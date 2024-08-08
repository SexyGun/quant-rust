use crate::db::stock_info::StockInfo;
use pyo3::prelude::*;

pub async fn get_all_stock_data() -> PyResult<Vec<StockInfo>> {
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
        Ok(result)
    })
}
