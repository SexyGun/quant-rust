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
use rocket::tokio;
use rocket::tokio::time::{sleep, Duration as to_do};
use rocket_db_pools::diesel::{AsyncConnection, RunQueryDsl};
use rocket_db_pools::Connection;
use std::ops::DerefMut;
// 导入 Rocket 的 Debug 类型，用于调试错误响应。
use rocket::serde::{Deserialize, Serialize};
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

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct DailyRes {
    request_id: String,
    code: i32,
    msg: String,
    data: ResData,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct ResData {
    fields: Vec<String>,
    items: Vec<(
        String,         // ts_code
        Option<String>, // trade_date
        Option<f64>,    // open
        Option<f64>,    // high
        Option<f64>,    // low
        Option<f64>,    // close
        Option<f64>,    // pre_close
        Option<f64>,    // change
        Option<f64>,    // pct_chg
        Option<f64>,    // vol
        Option<f64>,    // amount
    )>, // Vec<()>)
    has_more: bool,
}
#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct DailyReq {
    api_name: String,
    token: String,
    params: Params,
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Params {
    ts_code: String,
    start_date: String,
    end_date: String,
}

/// 获取单只股票一段时间内的价格数据
/// ts_code：股票代码
/// date_range：（开始时间，结束时间）时间范围
pub async fn get_stock_price_data(
    ts_code: String,
    date_range: (String, String),
) -> Result<Vec<StockPriceInfo>, reqwest::Error> {
    let (stock_start_time, stock_end_time) = date_range;

    let client = reqwest::Client::new();
    let res = client
        .post("http://api.tushare.pro")
        .json(&DailyReq {
            api_name: "daily".to_string(),
            token: "---".to_string(),
            params: Params {
                ts_code,
                start_date: stock_start_time,
                end_date: stock_end_time,
            },
        })
        .send()
        .await?;
    let result: DailyRes = res.json().await?;
    let stock_list: Vec<StockPriceInfo> = result
        .data
        .items
        .into_iter()
        .map(|item| StockPriceInfo::from(item))
        .collect();
    Ok(stock_list)
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
    range: Option<usize>,
) -> Result<()> {
    let rps_range = range.unwrap_or(120);
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
    // 服务器为2核心，在处理任务时出现了连接超时的问题，一共出现了3次，所以这里设置为 5
    for offset in 0..5 {
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
                if (idx % 5) == offset {
                    let db_conn = db_conn.deref_mut();
                    match get_local_stock_price_data(db_conn, code.to_string()).await {
                        Ok(stock_basic_info_list) => {
                            let mut last_date_index =
                                stock_basic_info_list.iter().position(|stock| {
                                    stock.trade_date.as_ref().unwrap() == today_str.as_str()
                                });
                            let mut cur_today_str = today_str.to_string();
                            // 股票周六日不交易，所以当天的数据不存在Ç
                            // 如果当天的数据不存在，向前查找
                            while last_date_index.is_none() {
                                // 定义格式化字符串
                                let format = "%Y%m%d";
                                let cur_date =
                                    NaiveDate::parse_from_str(&cur_today_str, format).unwrap();
                                let prev_date =
                                    cur_date.pred_opt().unwrap().format(format).to_string();
                                last_date_index = stock_basic_info_list.iter().position(|stock| {
                                    stock.trade_date.as_ref().unwrap() == prev_date.as_str()
                                });
                                cur_today_str = prev_date;
                            }
                            if last_date_index.is_none() {
                                eprintln!("没有找到当天的数据");
                                continue;
                            }
                            let before_stock = if last_date_index.unwrap() > rps_range {
                                stock_basic_info_list[last_date_index.unwrap() - rps_range].clone()
                            } else {
                                stock_basic_info_list[0].clone()
                            };
                            let now_stock = stock_basic_info_list[last_date_index.unwrap()].clone();
                            result.push(StockIncrease {
                                ts_code: code.to_string(),
                                increase: (now_stock.close.unwrap() - before_stock.close.unwrap())
                                    / before_stock.close.unwrap()
                                    * 100.0,
                                trade_date: Some(today_str.to_string()),
                            });
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
    } else {
        need_insert = all_stock;
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
pub async fn get_local_stock_price_data(
    conn: &mut AsyncMysqlConnection,
    ts_code: String,
) -> Result<Vec<StockPriceInfo>> {
    Ok(stock_daily_info::table
        .filter(stock_daily_info::ts_code.eq(ts_code))
        .load(conn)
        .await?)
}
