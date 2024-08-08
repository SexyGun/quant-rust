use rocket::fairing::AdHoc;
use rocket::response::Debug; // 导入 Rocket 的 Debug 类型，用于调试错误响应。
use rocket_db_pools::diesel::AsyncConnection; // 导入 AsyncConnection 用于与 MySQL 数据库异步交互。
use rocket_db_pools::diesel::RunQueryDsl;
use rocket_db_pools::Connection;

use crate::db::schema::stock_info_list;
use crate::db::{connection::Db, stock_info::StockInfo};
use crate::stock_lib::get_all_stock_list;

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

pub fn stage() -> AdHoc {
    // AdHoc::on_ignite 是 Rocket 提供的一种机制，
    // 用于在 Rocket 启动时执行自定义的初始化代码。这个方法接受两个参数：
    // 名称：一个字符串，用于标识这个阶段的名称，通常用于日志或调试信息。
    // 初始化闭包：一个异步闭包（async {}），用于执行初始化代码
    AdHoc::on_ignite("Route Stock Stage", |rocket| async {
        rocket
            // .attach(Db::init())
            .mount("/stock", routes![get_basic_info])
    })
}
