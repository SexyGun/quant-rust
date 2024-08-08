#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket_cors::{AllowedOrigins, CorsOptions};

mod diesel_mysql;
use back_end::stock_lib::get_all_stock_list;

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
        .attach(diesel_mysql::stage())
        .attach(back_end::routes::stock::stage())
        .mount("/data", routes![test])
}
