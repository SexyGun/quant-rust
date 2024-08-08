use pyo3::prelude::*;

// 这里用 Option 是因为接口返回不一定有值，因此需要用 Option 来接一下
#[derive(FromPyObject, Debug)]
#[pyo3(from_item_all)]
pub struct StockInfo {
    ts_code: Option<String>,            // TS代码
    symbol: Option<String>,             // 股票代码
    name: Option<String>,               // 股票名称
    area: Option<String>,               // 地域
    industry: Option<String>,           // 所属行业
    cnspell: Option<String>,            // 拼音缩写
    market: Option<String>,             // 市场类型（主板/创业板/科创板/CDR）
    list_date: Option<String>,          // 上市日期
    act_name: Option<String>,           // 实控人名称
    act_ent_type: Option<String>,       // 实控人企业性质
}

pub fn get_all_stock_data() -> PyResult<()> {
    println!("1212");
    // 初始化 Python 解释器
    pyo3::prepare_freethreaded_python();
    // Python::with_gil 是一个用于获取 Python 全局解释器锁（GIL）的帮助器。
    // Python 的 GIL 是一个全局锁，用于保证在同一时间只有一个线程可以执行 Python 代码。
    // with_gil 会在闭包中自动管理 GIL。
    Python::with_gil(|py| {
        let code = r#"
import tushare as ts
my_ts_token = '5ad981f92afb58c9b91c87a22d8e03417e13a7af1cbe458aecf95164'
ts.set_token(my_ts_token)
pro = ts.pro_api()

def GetAllStockData():
    stock_list = pro.stock_basic()
    return stock_list.to_dict(orient="records")
"#;
        let fun = PyModule::from_code_bound(py, code, "", "")?.getattr("GetAllStockData")?;
        let result: Vec<StockInfo> = fun.call0()?.extract()?;
        println!("{:?}", result);
        Ok(())
    })
}
