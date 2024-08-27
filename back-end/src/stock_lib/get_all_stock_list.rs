use crate::db::stock_info::StockInfo;
use rocket::serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct StockRes {
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
        String,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
    )>,
    has_more: bool,
}

pub async fn get_all_stock_data() -> Result<Vec<StockInfo>, reqwest::Error> {
    let mut map = HashMap::new();
    map.insert("api_name", "stock_basic");
    map.insert(
        "token",
        "5ad981f92afb58c9b91c87a22d8e03417e13a7af1cbe458aecf95164",
    );

    let client = reqwest::Client::new();
    let res = client
        .post("http://api.tushare.pro")
        .json(&map)
        .send()
        .await?;
    let result: StockRes = res.json().await?;
    let stock_list: Vec<StockInfo> = result
        .data
        .items
        .into_iter()
        .map(|item| StockInfo::from(item))
        .collect();
    Ok(stock_list)
}
