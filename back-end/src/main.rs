#[macro_use]
extern crate rocket;

use rocket::serde::Serialize;
use rocket::serde::json::Json;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct MyResponse {
    code: i32,
    data: String,
    msg: String,
}

#[get("/")]
fn test() -> Json<MyResponse> {
    Json(MyResponse {
        code: 0,
        data: "hello".to_string(),
        msg: "ok".to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/data", routes![test])
}
