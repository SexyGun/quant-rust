use crate::db::schema::{rps_values, stock_daily_info, stock_info_list};
use crate::db::{
    connection::Db,
    stock_info::{StockPriceInfo, StockRps},
};
use crate::AppErrorEnum;
use chrono::{Duration, NaiveDate, Utc};
use diesel::dsl::{count_star, max, min};
use diesel::{ExpressionMethods, QueryDsl};
use ndarray::Array1;
use pyo3::prelude::*;
use rocket::tokio;
use rocket::tokio::time::{sleep, Duration as to_do};
use rocket_db_pools::diesel::{AsyncConnection, RunQueryDsl};
use rocket_db_pools::Connection;
use std::ops::DerefMut;
// 导入 Rocket 的 Debug 类型，用于调试错误响应。
use rocket::State;
use std::sync::Arc;

/**
* RPS指标（Relative Price Strength Rating），即股价相对强度指标,
* 根据一段时间内个股涨幅在全部股票涨幅排名中的位次值，选取出市场中的强势股。
*
* 计算 RPS 的步骤
   1.	计算涨幅：
       对每只股票在指定时间段（如 120 日）的涨幅进行计算。
   2.	排名：
       根据计算出的涨幅，对所有股票进行排名，确定每只股票在所有股票中的相对位置。
   3.	计算 RPS：
       将排名转换为一个百分比，通常在 1 到 100 之间。排名前 1% 的股票将获得接近 100 的 RPS 值

* 具体计算公式
* 	1.	计算涨幅：
*      对每只股票(i)在时间段(T)内的涨幅 ( return_i ) 计算公式为:
*      return_i = (p_end_i - p_start_i) / p_start_i * 100
*   2.  排名
*      对所有股票的涨幅进行排序，得到每只股票的排名
* 	3.	计算百分比排名，即 RPS：
*      计算每只股票的百分比排名
*      percent_rank_i = (rank_i - 1) / (total_stocks - 1) * 100
*      其中 (rank_i) 是股票 i 的排名（从 1 开始），total_stocks 是所有股票的总数。
*/

// 定义一个通用的 Result 类型，默认错误类型为 AppErrorEnum，用于处理 col_stock_rps 中的错误。
type Result<T, E = AppErrorEnum> = std::result::Result<T, E>;
use pyo3::types::PyList;
use std::path::Path;
/// 获取单只股票一段时间内的价格数据
/// ts_code：股票代码
/// date_range：（开始时间，结束时间）时间范围
///
/// let code = r#"
/// import tushare as ts
/// my_ts_token = '5ad981f92afb58c9b91c87a22d8e03417e13a7af1cbe458aecf95164'
/// ts.set_token(my_ts_token)
/// pro = ts.pro_api()

/// def GetStockDataApi(*args, **kwargs):
///     return []
///     # 获取股票原始数据
///     df_stockload = pro.daily(
///         ts_code=args[0], start_date=20240814, end_date=args[2]
///     )
///     if df_stockload.empty:
///         return []
///     return df_stockload.to_dict(orient='records')
///         "#;
/// let fun: Py<PyAny> = PyModule::from_code_bound(py, &code, "", "")
///     .expect("from_code_bound err")
///     .getattr("GetStockDataApi")?
///     .into();
/// 以上是原实现方法，其中使用了 from_code_bound 方法，这个方法的官方说明为：
/// PyModule::from_code_bound can be used to generate a Python module which can then be used just as if it was imported with PyModule::import.
/// Q-Gpt：这个是原文指引，使用 from_code_bound 时，
/// 他会生成一个 Python module 此时如果我线程的并发量够快的情况下，
/// 是不是会出现其内部的文件IO 还没有处理完毕，我这边就先读取的情况，
/// 进而导致某一些线程会触发 PyErr { type: <class 'pandas.errors.EmptyDataError'>, value: EmptyDataError('No columns to parse from file'), traceback: Some(<traceback object at 0x1326f7980>) } 这个错误
/// A-Gpt：使用 PyModule::from_code_bound 在多线程环境下可能会遇到线程并发导致的文件 I/O 问题，特别是在 Python 模块依赖于外部文件或资源时
/// 如果你的 Python 模块确实需要依赖文件系统，并且这些文件可能会被多个线程访问，可以考虑以下策略：
/// - **将文件预先准备好**:
/// 确保所有文件在模块创建之前都已经准备好，并且文件内容是稳定的。
/// - **在每个线程中使用独立的 Python 环境**:
/// 如果可能，使用 `PyModule::import` 而不是 `PyModule::from_code_bound` 来避免因动态代码创建而引发的多线程问题。
/// - **使用 Python 内建的锁机制**:
/// 在 Python 代码中使用线程锁（例如 `threading.Lock`）来保护文件访问。
/// 通过这些策略，可以减少因多线程访问共享资源而导致的问题，提高程序的稳定性。
///
/// 结合答案，目前确认是使用 from_code_bound 会创建一个临时的 module，然后再去读这个文件，
/// 这样在多线程环境下，会存在还没有创建文件时就有新的线程进来要进行访问，导致错误。
pub async fn get_stock_price_data(
    ts_code: String,
    date_range: (String, String),
) -> PyResult<Vec<StockPriceInfo>> {
    let (stock_start_time, stock_end_time) = date_range;
    // 初始化 Python 解释器
    // Python::with_gil 是一个用于获取 Python 全局解释器锁（GIL）的帮助器。
    // Python 的 GIL 是一个全局锁，用于保证在同一时间只有一个线程可以执行 Python 代码。
    // with_gil 会在闭包中自动管理 GIL。
    Python::with_gil(|py| {
        // 将要使用的方法以静态的方式进行导入
        // Define the path to your Python module
        let module_dir = Path::new("/Users/lichen/workplace/quant-rust/back-end/src/py_tools");
        let module_name = "GetStockDataApi";
        // Add the module directory to sys.path
        // 在系统将你的 python 模块进行注册，以便后续进行导入
        let sys = PyModule::import_bound(py, "sys")?;
        let binding = sys.getattr("path")?;
        let path_list = binding.downcast::<PyList>()?;
        if path_list.get_item(0).unwrap().extract::<&str>().unwrap() != module_dir.to_str().unwrap()
        {
            // 插入操作可以优化为如果有就不插入
            path_list.insert(0, module_dir.to_str().unwrap())?;
        }
        // Import the Python module by its name
        let my_module = PyModule::import_bound(py, module_name)?;
        // Get the function from the module
        let get_stock_data_api = my_module.getattr("GetStockDataApi")?;

        let args = (ts_code, stock_start_time, stock_end_time);
        let result: Vec<StockPriceInfo> = get_stock_data_api.call1(args)?.extract()?;
        Ok(result)
    })
}

#[derive(Debug)]
struct StockIncrease {
    // 股票涨幅
    ts_code: String,
    increase: f64,
    trade_date: Option<String>,
}
// 计算股票的 RPS
pub async fn col_stock_rps(
    mut db: Connection<Db>,
    db_state: &State<Db>,
    end_date: Option<String>,
) -> Result<()> {
    let code_list = stock_info_list::table
        .select(stock_info_list::ts_code)
        .load::<String>(&mut db)
        .await?;
    let list_len = code_list.len();
    // 获取当前日期
    let today = Utc::now().date_naive();
    // 定义格式化字符串
    let format = "%Y%m%d";
    let today_str = end_date.unwrap_or(today.format(format).to_string());
    // 检查数据库中是否有当天的数据，有则直接返回，不执行后续操作
    let has_data = rps_values::table
        .filter(rps_values::trade_date.eq(today_str.clone()))
        .select(count_star())
        .first::<i64>(&mut db)
        .await?
        != 0;
    if has_data {
        return Ok(());
    }
    /*
    Arc<Mutex<T>> 会导致线程在访问共享数据时需要获取锁，这可能导致线程的并发性能下降。如果锁的争用很严重，线程可能会被迫等待，这样看起来像是同步执行。
    为了减少锁的争用，可以尝试以下几种方法：
    •	减少锁的粒度：在 thread::spawn 内部尽量减少对锁的持有时间，只在需要的时候锁定数据。
    •	使用无锁数据结构：考虑使用无锁数据结构（如 crossbeam 提供的无锁队列）来代替 Mutex。

    因此这里去掉了锁，其实锁本来也没用😂
     */
    let share_code_list = Arc::new(code_list);
    // 使用 Arc 共享日期字符串
    let today_str = Arc::new(today_str);
    // 线程任务队列
    let mut tasks = vec![];
    // 线程数最好不超过本机的 CPU 核心数
    // 我的电脑为 11 核心，所以这里设置为 10
    for offset in 0..10 {
        // 启动 10 个线程
        let today_str = Arc::clone(&today_str);
        let code_list = Arc::clone(&share_code_list);
        // db_state 是一个 Rocket 的 State 类型，用于存储全局状态
        // 这里存储的是数据库连接
        // 通过 db_state.get 获取数据库连接
        let mut db_conn = db_state.get().await.expect("db connect err");

        let task = tokio::spawn(async move {
            let mut result = vec![];
            for (idx, code) in code_list.iter().enumerate() {
                if (idx % 10) == offset {
                    let db_conn = db_conn.deref_mut();
                    match get_local_stock_price_data(db_conn, code.to_string()).await {
                        Ok(stock_basic_info_list) => {
                            if let (Some(before_stock), Some(now_stock)) = (
                                <[StockPriceInfo]>::first(&stock_basic_info_list),
                                stock_basic_info_list.last(),
                            ) {
                                result.push(StockIncrease {
                                    ts_code: code.to_string(),
                                    increase: (now_stock.close.unwrap()
                                        - before_stock.close.unwrap())
                                        / before_stock.close.unwrap()
                                        * 100.0,
                                    trade_date: Some(today_str.to_string()),
                                });
                            } else {
                                println!("三无 Code: {:?}", code);
                            }
                        }
                        Err(e) => eprintln!("Error fetching stock data: {:?}", e),
                    }
                }
            }
            result
        });

        tasks.push(task);
    }
    let mut all_increase: Vec<StockIncrease> = Vec::with_capacity(list_len);
    for task in tasks {
        let mut res: Vec<StockIncrease> = task.await?;
        all_increase.append(&mut res);
    }
    // 对所有股票的涨幅进行排序，得到每只股票的排名
    all_increase.sort_by(|v_1, v_2| v_1.increase.partial_cmp(&v_2.increase).unwrap());
    let stock_rank_list: Vec<f64> = all_increase
        .iter()
        .enumerate()
        .map(|(idx, _)| (idx + 1) as f64)
        .collect();
    let vector_list = Array1::from(stock_rank_list);
    // percent_rank_i = (rank_i - 1) / (total_stocks - 1) * 100
    let percent_rank = (&vector_list - 1.0) / (all_increase.len() - 1) as f64 * 100.0;
    let mut stock_rps: Vec<StockRps> = Vec::with_capacity(all_increase.len());
    for (idx, stock) in all_increase.iter().enumerate() {
        if idx >= all_increase.len() - 300 {
            // 取排名前 300 的票，以及得到整个盘面的 rps
            stock_rps.push(StockRps {
                ts_code: stock.ts_code.to_string(),
                trade_date: stock.trade_date.clone(),
                increase: Some(stock.increase),
                rps: Some(percent_rank[idx]),
            })
        }
    }
    db.transaction(|mut conn| {
        Box::pin(async move {
            diesel::insert_into(rps_values::table)
                .values(&stock_rps)
                .execute(&mut conn)
                .await?;
            Ok::<_, diesel::result::Error>(())
        })
    })
    .await?;
    Ok(())
}
use rayon::prelude::*;
// 获取股票的价格数据
pub async fn fetch_stock_daily_range(
    mut db: Connection<Db>,
    closing_date: Option<String>,
    range: Option<i64>,
) -> Result<()> {
    let code_list = stock_info_list::table
        .select(stock_info_list::ts_code)
        .load::<String>(&mut db)
        .await?;
    let list_len = code_list.len();
    /*
    Arc<Mutex<T>> 会导致线程在访问共享数据时需要获取锁，这可能导致线程的并发性能下降。如果锁的争用很严重，线程可能会被迫等待，这样看起来像是同步执行。
    为了减少锁的争用，可以尝试以下几种方法：
    •	减少锁的粒度：在 thread::spawn 内部尽量减少对锁的持有时间，只在需要的时候锁定数据。
    •	使用无锁数据结构：考虑使用无锁数据结构（如 crossbeam 提供的无锁队列）来代替 Mutex。

    因此这里去掉了锁，其实锁本来也没用😂
     */
    let share_code_list = Arc::new(code_list);
    // 定义格式化字符串
    let format = "%Y%m%d";

    // 获取当前日期
    let today = if let Some(closing_date) = closing_date {
        NaiveDate::parse_from_str(&closing_date, &format).expect("日期解析失败")
    } else {
        Utc::now().date_naive()
    };
    // 默认获取 120 天的数据
    let range = range.unwrap_or(120);
    // 计算 range 天前的日期
    let past_date = today - Duration::days(range);
    // 格式化日期
    let today_str = today.format(format).to_string();
    let past_date_str = past_date.format(format).to_string();
    // 使用 Arc 共享日期字符串
    let today_str = Arc::new(today_str);
    let past_date_str = Arc::new(past_date_str);
    // 线程任务队列
    let mut tasks = vec![];
    for offset in 0..10 {
        // 启动 10 个线程
        let today_str = Arc::clone(&today_str);
        let past_date_str = Arc::clone(&past_date_str);
        let code_list = Arc::clone(&share_code_list);

        let task = tokio::spawn(async move {
            let mut result = vec![];
            for (idx, code) in code_list.iter().enumerate() {
                if (idx % 10) == offset {
                    match get_stock_price_data(
                        code.to_string(),
                        (past_date_str.to_string(), today_str.to_string()),
                    )
                    .await
                    {
                        Ok(mut stock_basic_info_list) => {
                            println!(
                                "code: {:?}, index: {:?}, length: {:?}",
                                code,
                                idx,
                                stock_basic_info_list.len()
                            );
                            // 将获取的数据转移到 result 中
                            result.append(&mut stock_basic_info_list);
                        }
                        Err(e) => eprintln!("Error fetching stock data: {:?}", e),
                    }
                }
                // 获取股票的接口一分钟只能调用 1000 次，这里开启 10 个线程
                // 每调用一次接口后，暂停 500 毫秒 约等于 1min 最多调用 1200 次
                // 再算上接口调用的时间，怎么也够了
                // 实测 500 有点慢更新为 100
                let _ = sleep(to_do::from_millis(100)).await;
            }
            result
        });

        tasks.push(task);
    }
    let mut all_stock: Vec<StockPriceInfo> = Vec::with_capacity(list_len);
    for task in tasks {
        let mut res: Vec<StockPriceInfo> = task.await?;
        all_stock.append(&mut res);
    }
    // 这里为什么不能用 on_conflict().do_update().set() 方法呢?
    // 因为 set 时需要穿入一个实现了 AsChangeset 的类型，而 &[StockPriceInfo] 是一个 Vec 并没有实现 AsChangeset
    // 或者可以这么说 set 只能 set 一条单行的数据，而这里是多行数据
    // 所以我这里需要对 all_stock 做进一步的处理
    // 数据库中的结束日期，即最新的日期
    let start_date = stock_daily_info::table
        .select(min(stock_daily_info::trade_date))
        .first::<Option<String>>(&mut db)
        .await?;
    // 数据库中的开始日期，即最早的日期
    let end_date = stock_daily_info::table
        .select(max(stock_daily_info::trade_date))
        .first::<Option<String>>(&mut db)
        .await?;
    // 需要插入的数据
    let mut need_insert: Vec<StockPriceInfo> = vec![];

    // 如果数据库中没有数据，直接插入
    if let (Some(start), Some(end)) = (start_date, end_date) {
        // 如果数据库中有数据，需要对 all_stock 进行处理
        // 使用 filter 将 all_stock 中的数据按照日期进行过滤
        // 只保留数据库中没有的数据
        need_insert = all_stock
            .into_par_iter()
            .filter(|stock| {
                stock.trade_date.as_ref().unwrap() < &start
                    || stock.trade_date.as_ref().unwrap() > &end
            })
            .collect();
    }
    println!("need_insert: {:?}", need_insert.len());
    // 如果 need_insert 为空，直接返回
    if need_insert.is_empty() {
        return Ok(());
    }
    db.transaction(|mut conn| {
        Box::pin(async move {
            for batch in need_insert.chunks(5000) {
                diesel::insert_into(stock_daily_info::table)
                    .values(batch)
                    .execute(&mut conn)
                    .await?;
            }
            Ok::<_, diesel::result::Error>(())
        })
    })
    .await?;
    Ok(())
}

use rocket_db_pools::diesel::AsyncMysqlConnection;
// 获取本地股票价格数据
async fn get_local_stock_price_data(
    conn: &mut AsyncMysqlConnection,
    ts_code: String,
) -> Result<Vec<StockPriceInfo>> {
    Ok(stock_daily_info::table
        .filter(stock_daily_info::ts_code.eq(ts_code))
        .load(conn)
        .await?)
}
