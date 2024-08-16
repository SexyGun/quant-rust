#[macro_use]
extern crate rocket;

use rocket::serde::Serialize;
use rocket::{fairing::AdHoc, serde::json::Json};
use rocket_cors::{AllowedOrigins, CorsOptions};

use back_end::{db::connection::Db, stock_lib::get_all_stock_list};
use rocket_db_pools::Database; // 导入 Rocket 数据库池的 Connection 和 Database 类型。

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct MyResponse {
    code: i32,
    data: String,
    msg: String,
}

#[get("/")]
async fn test() -> Json<MyResponse> {
    get_all_stock_list::get_all_stock_data().await.unwrap();
    Json(MyResponse {
        code: 0,
        data: "hello".to_string(),
        msg: "ok".to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    // 定义允许的跨域源
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://127.0.0.1",
        "http://112.74.46.63",
        "http://127.0.0.1:5173",
        "http://localhost:5173",
    ]);

    // 配置 CORS 选项
    let cors = CorsOptions {
        allowed_origins,
        ..Default::default()
    }
    .to_cors()
    .unwrap(); // `unwrap` 是为了简化示例，实际应用中应处理可能的错误

    // 在 Rust 程序中，通常在程序的初始化阶段（例如在 main 函数或应用启动时）调用一次 pyo3::prepare_freethreaded_python()。
    // 这是确保整个程序生命周期内 Python 解释器的多线程支持正确配置的最佳实践。
    // 初始化后，可以安全地在程序的其他部分（包括异步函数和多线程任务）中调用 Python 代码。
    pyo3::prepare_freethreaded_python();
    rocket::build()
        .attach(cors)
        .attach(AdHoc::on_ignite("Db Init Stage", |rocket| async {
            rocket.attach(Db::init())
        }))
        .attach(back_end::routes::stock::stage())
        .mount("/data", routes![test])
}
