use crate::db::schema::{rps_values, stock_daily_info, stock_info_list};
use rocket::serde::{Deserialize, Serialize};
use std::convert::From;

// 这里用 Option 是因为接口返回不一定有值，因此需要用 Option 来接一下
#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, AsChangeset)]
#[serde(crate = "rocket::serde")] // 指定 serde 使用 Rocket 自带的 serde 库，而不是默认的 serde。
#[diesel(table_name=stock_info_list)] // 指定 Diesel 中表的名称为 stock_info
pub struct StockInfo {
    pub ts_code: String,              // TS代码(主键)
    pub symbol: Option<String>,       // 股票代码
    pub name: Option<String>,         // 股票名称
    pub area: Option<String>,         // 地域
    pub industry: Option<String>,     // 所属行业
    pub cnspell: Option<String>,      // 拼音缩写
    pub market: Option<String>,       // 市场类型（主板/创业板/科创板/CDR）
    pub list_date: Option<String>,    // 上市日期
    pub act_name: Option<String>,     // 实控人名称
    pub act_ent_type: Option<String>, // 实控人企业性质
}

impl
    From<(
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
    )> for StockInfo
{
    fn from(
        item: (
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
        ),
    ) -> Self {
        StockInfo {
            ts_code: item.0,      // TS代码(主键)
            symbol: item.1,       // 股票代码
            name: item.2,         // 股票名称
            area: item.3,         // 地域
            industry: item.4,     // 所属行业
            cnspell: item.5,      // 拼音缩写
            market: item.6,       // 市场类型（主板/创业板/科创板/CDR）
            list_date: item.7,    // 上市日期
            act_name: item.8,     // 实控人名称
            act_ent_type: item.9, // 实控人企业性质
        }
    }
}

#[derive(Debug, Clone, Queryable, Insertable, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")] // 指定 serde 使用 Rocket 自带的 serde 库，而不是默认的 serde。
#[diesel(table_name=stock_daily_info)] // 指定 Diesel 中表的名称为 stock_info
pub struct StockPriceInfo {
    pub ts_code: String,            // 股票代码
    pub trade_date: Option<String>, // 交易日期
    pub open: Option<f64>,          // 开盘价
    pub close: Option<f64>,         // 收盘价
    pub high: Option<f64>,          // 最高价
    pub low: Option<f64>,           // 最低价
    pub pre_close: Option<f64>,     // 昨收价【除权价，前复权】
    pub vol: Option<f64>,           // 成交量 （手）
    pub change: Option<f64>,        // 涨跌额
    pub pct_chg: Option<f64>, // 涨跌幅 【基于除权后的昨收计算的涨跌幅：（今收-除权昨收）/除权昨收 】
    pub amount: Option<f64>,  // 成交额 （千元）
}
impl
    From<(
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
    )> for StockPriceInfo
{
    fn from(
        value: (
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
        ),
    ) -> Self {
        StockPriceInfo {
            ts_code: value.0,
            trade_date: value.1,
            open: value.2,
            high: value.3,
            low: value.4,
            close: value.5,
            pre_close: value.6,
            change: value.7,
            pct_chg: value.8,
            vol: value.9,
            amount: value.10,
        }
    }
}
// 这里用 Option 是因为接口返回不一定有值，因此需要用 Option 来接一下
#[derive(Debug, Clone, Queryable, Insertable)]
#[diesel(table_name=rps_values)] // 指定 Diesel 中表的名称为 stock_info
pub struct StockRps {
    pub ts_code: String,            // 股票代码
    pub trade_date: Option<String>, // 交易日期
    pub rps: Option<f64>,           // 股价强度指数
    pub increase: Option<f64>,      // 指定时间涨幅
}
