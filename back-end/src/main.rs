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
    rocket::build()
        .attach(cors)
        .attach(AdHoc::on_ignite("Db Init Stage", |rocket| async {
            rocket.attach(Db::init())
        }))
        .attach(back_end::routes::stock::stage())
        .mount("/data", routes![test])
}
