use pyo3::prelude::*;
use crate::db::schema::stock_info_list;
use rocket::serde::{Deserialize, Serialize};

// 这里用 Option 是因为接口返回不一定有值，因此需要用 Option 来接一下
#[derive(FromPyObject, Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")] // 指定 serde 使用 Rocket 自带的 serde 库，而不是默认的 serde。
#[pyo3(from_item_all)]
#[diesel(table_name=stock_info_list)] // 指定 Diesel 中表的名称为 stock_info
pub struct StockInfo {
    pub ts_code: String,                    // TS代码(主键)
    pub symbol: Option<String>,             // 股票代码
    pub name: Option<String>,               // 股票名称
    pub area: Option<String>,               // 地域
    pub industry: Option<String>,           // 所属行业
    pub cnspell: Option<String>,            // 拼音缩写
    pub market: Option<String>,             // 市场类型（主板/创业板/科创板/CDR）
    pub list_date: Option<String>,          // 上市日期
    pub act_name: Option<String>,           // 实控人名称
    pub act_ent_type: Option<String>,       // 实控人企业性质
}