use crate::db::schema::{rps_values, stock_info_list};
use crate::db::{connection::Db, stock_info::StockInfo};
use crate::stock_lib::{
    get_all_stock_list, get_stock_rps_list,
    stock_trade::{simulate_stock_trade, OperateRecord, TradeResult},
};
use diesel::{ExpressionMethods, QueryDsl};
use rocket::fairing::AdHoc;
use rocket::response::Debug; // 导入 Rocket 的 Debug 类型，用于调试错误响应。
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio;
use rocket::State;
use rocket_db_pools::diesel::AsyncConnection; // 导入 AsyncConnection 用于与 MySQL 数据库异步交互。
use rocket_db_pools::diesel::RunQueryDsl;
use rocket_db_pools::Connection;

// 定义一个通用的 Result 类型，默认错误类型为 Debug<diesel::result::Error>，用于处理数据库操作中的错误。
type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[get("/basic")]
async fn get_basic_info(mut db: Connection<Db>) -> Result<()> {
    let stock_basic_info_list: Vec<StockInfo> = get_all_stock_list::get_all_stock_data()
        .await
        .expect("获取基础数据失败");
    // db.transaction 的作用是：
    // 1.保证操作原子性：所有在事务中执行的数据库操作要么全部成功，要么全部回滚，这样可以保证数据库的状态一致性。
    // 2.捕获错误：如果事务中的任何操作失败（例如，插入失败），整个事务会回滚，确保数据库不会处于不一致的状态。
    // 3.封装操作：将多个数据库操作封装在一个事务中，使得这些操作要么成功，要么失败，不会对数据库造成部分更新的影响。
    db.transaction(|mut conn| {
        // Box::pin 保证这段代码在内容中的位置不会被变动，以致异步操作完事后找不到这部分值
        Box::pin(async move {
            diesel::insert_into(stock_info_list::table)
                .values(&stock_basic_info_list)
                .execute(&mut conn)
                .await?;
            Ok::<_, diesel::result::Error>(())
        })
    })
    .await?;
    Ok(())
}
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct ReqFetchStockRps {
    date: Option<String>,
}

#[post("/fetch_stock_rps_list", data = "<req>")]
async fn get_stock_rps(
    db: Connection<Db>,
    db_state: &State<Db>,
    req: Json<ReqFetchStockRps>,
) -> Result<()> {
    match get_stock_rps_list::col_stock_rps(db, db_state, req.date.clone()).await {
        Ok(()) => {}
        Err(e) => {
            println!("{:?}", e)
        }
    };
    Ok(())
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct ReqFetchStockDailyRange {
    closing_date: Option<String>,
    range: Option<i64>,
}

use rocket::http::Status;
use rocket::response::status;

#[post("/fetch_stock_daily_range", data = "<req>")]
async fn get_stock_daily_range(
    db: Connection<Db>,
    req: Json<ReqFetchStockDailyRange>,
) -> status::Custom<&'static str> {
    // 同步执行
    // match get_stock_rps_list::fetch_stock_daily_range(
    //     db,
    //     req.closing_date.clone(),
    //     req.range.clone(),
    // )
    // .await
    // {
    //     Ok(()) => {}
    //     Err(e) => {
    //         println!("{:?}", e)
    //     }
    // };

    // tokio::spawn函数内的get_stock_rps_list::fetch_stock_daily_range函数会在新的异步任务中执行。这个任务是立即被安排在Tokio运行时上的，所以你可以认为它已经开始执行了。
    tokio::spawn(async move {
        match get_stock_rps_list::fetch_stock_daily_range(
            db,
            req.closing_date.clone(),
            req.range.clone(),
        )
        .await
        {
            Ok(()) => {}
            Err(e) => {
                println!("{:?}", e)
            }
        };
    });

    // 立即返回一个消息给客户端
    status::Custom(
        Status::Accepted,
        "The request is being processed. Please wait.",
    )
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct RpsRequest {
    date: Option<String>,
}
#[derive(Serialize, Deserialize, Queryable)]
#[serde(crate = "rocket::serde")]
struct RpsResponse {
    ts_code: String,
    name: Option<String>,
    rps: Option<f64>,
    increase: Option<f64>,
}

#[post("/rps-top", data = "<search>")]
async fn get_stock_rps_top(
    mut db: Connection<Db>,
    search: Json<RpsRequest>,
) -> Result<Json<Vec<RpsResponse>>> {
    if let Some(date) = &search.date {
        let result: Vec<RpsResponse> = stock_info_list::table
            .inner_join(rps_values::table)
            .filter(rps_values::trade_date.eq(date.to_string()))
            .select((
                rps_values::ts_code,
                stock_info_list::name,
                rps_values::rps,
                rps_values::increase,
            ))
            .order(rps_values::rps.desc())
            .load(&mut db)
            .await?;
        return Ok(Json(result));
    }
    Ok(Json(vec![]))
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct PaginationStockInfo {
    ts_code: Option<String>, // TS代码(主键)
    symbol: Option<String>,  // 股票代码
    name: Option<String>,    // 股票名称
    area: Option<String>,    // 地域
    current: i64,            // 当前页数
    size: i64,               // 页面大小
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct ResStock {
    data: Vec<StockInfo>,
    current: i64,
    size: i64,
    total: i64,
}

#[post("/query", data = "<search>")]
async fn query_basic(
    mut db: Connection<Db>,
    search: Json<PaginationStockInfo>,
) -> Result<Json<ResStock>> {
    let mut query = stock_info_list::table.into_boxed(); // 将查询转为动态构建模式
    if let Some(ts_code) = &search.ts_code {
        query = query.filter(stock_info_list::ts_code.eq(ts_code));
    }
    if let Some(symbol) = &search.symbol {
        query = query.filter(stock_info_list::symbol.eq(symbol));
    }
    if let Some(name) = &search.name {
        query = query.filter(stock_info_list::name.eq(name));
    }
    if let Some(area) = &search.area {
        query = query.filter(stock_info_list::area.eq(area));
    }
    // 计算分页参数
    let offset = (search.current - 1) * search.size;
    // 执行查询并分页
    let result: Vec<StockInfo> = query
        .limit(search.size)
        .offset(offset)
        .load::<StockInfo>(&mut db)
        .await?;
    use diesel::dsl::count_star;
    // 使用 count_star 函数来计算总数
    let total = stock_info_list::table
        .select(count_star())
        .first(&mut db)
        .await?;
    Ok(Json(ResStock {
        data: result,
        current: search.current,
        size: search.size,
        total,
    }))
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct SimulateReq {
    code: String,                         // 股票代码
    assets: Option<f64>,                  // 初始资金
    n1_range: Option<(usize, usize)>,     // N1范围
    n2_range: Option<(usize, usize)>,     // N2范围
    win_range: Option<(f64, f64)>,        // 盈利范围
    loss_range: Option<(f64, f64)>,       // 亏损范围
    adjust_range: Option<(usize, usize)>, // 调整范围
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct SimulateRes {
    df_stock: Vec<TradeResult>,
    operate_record: Vec<OperateRecord>,
    best_param: (
        Option<usize>,
        Option<usize>,
        Option<f64>,
        Option<f64>,
        Option<usize>,
    ),
}
#[post("/simulate", data = "<req>")]
async fn stock_simulate(db: Connection<Db>, req: Json<SimulateReq>) -> Result<Json<SimulateRes>> {
    let res = simulate_stock_trade(
        db,
        vec![req.code.clone()],
        req.assets.unwrap_or(100000.0),
        None,
        None,
        req.n1_range.clone(),
        req.n2_range.clone(),
        req.win_range.clone(),
        req.loss_range.clone(),
        req.adjust_range.clone(),
    )
    .await;
    let (code_result, best_param) = res.get(&req.code).unwrap().clone();
    Ok(Json(SimulateRes {
        df_stock: code_result.0,
        operate_record: code_result.1,
        best_param,
    }))
}

pub fn stage() -> AdHoc {
    // AdHoc::on_ignite 是 Rocket 提供的一种机制，
    // 用于在 Rocket 启动时执行自定义的初始化代码。这个方法接受两个参数：
    // 名称：一个字符串，用于标识这个阶段的名称，通常用于日志或调试信息。
    // 初始化闭包：一个异步闭包（async {}），用于执行初始化代码
    AdHoc::on_ignite("Route Stock Stage", |rocket| async {
        rocket.mount(
            "/stock",
            routes![
                get_basic_info,
                query_basic,
                get_stock_rps,
                get_stock_rps_top,
                get_stock_daily_range,
                stock_simulate,
            ],
        )
    })
}
