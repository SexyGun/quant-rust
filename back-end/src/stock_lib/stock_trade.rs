use std::collections::{HashMap, VecDeque};
use std::usize;

use crate::db::connection::Db;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;

use crate::db::stock_info::StockPriceInfo;
use crate::stock_lib::get_stock_rps_list::get_local_stock_price_data;
use rand::Rng;
use ta::indicators::{AverageTrueRange as ATR, Maximum, Minimum};
use ta::{DataItem, Next};

// 股票交易类
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
enum OrderType {
    Buy,  // 买入
    Sell, // 卖出
}
const DEFAULT_COMMISSION_COEFFICIENT: f64 = 0.00025; // 佣金系数, 默认 万2.5(float类型 0.00025)
const DEFAULT_TAX_COEFFICIENT: f64 = 0.001; // 印花税系数, 默认千 1 (float类型 0.001)

/// 股票账户类
#[derive(Clone)]
struct ST_Account {
    hold: HashMap<String, usize>, // 初始持有股票数
    cash: f64,                    // 初始现金
    commission_coefficient: f64,  // 佣金系数, 默认 万2.5(float类型 0.00025)
    tax_coefficient: f64,         // 印花税系数, 默认千 1 (float类型 0.001)
}

impl ST_Account {
    /// 生成一个新的股票账户
    /// hold: 初始持有股票数
    /// cash: 初始现金
    /// commission_coeff: 佣金系数
    /// tax_coeff: 印花税系数
    pub fn new(
        init_hold: HashMap<String, usize>,
        init_cash: f64,
        commission_coeff: f64,
        tax_coeff: f64,
    ) -> Self {
        ST_Account {
            hold: init_hold,
            cash: init_cash,
            commission_coefficient: commission_coeff,
            tax_coefficient: tax_coeff,
        }
    }
    /// 获取初始持有股票数
    /// code: 股票代码
    pub fn hold_available(&self, code: String) -> usize {
        if let Some(hold) = self.hold.get(&code) {
            *hold
        } else {
            0
        }
    }
    /// 获取初始现金
    pub fn cash_available(&self) -> f64 {
        self.cash
    }
    /// 获取最新资产
    pub fn latest_assets(&self, price: f64) -> f64 {
        let asset = self.cash;
        // 计算持有股票的市值
        // fold 函数的第一个参数是初始值, 第二个参数是一个闭包, 闭包的第一个参数是初始值, 第二个参数是迭代的值
        self.hold.values().fold(asset, |acc, v| {
            acc + *v as f64 * price * (1.0 - self.commission_coefficient - self.tax_coefficient)
        })
    }
    pub fn send_order(&mut self, code: String, amount: usize, price: f64, order_type: OrderType) {
        match order_type {
            OrderType::Buy => {
                // 更新资金剩余
                self.cash = self.cash
                    - price
                        * amount as f64
                        * (1.0 + self.commission_coefficient + self.tax_coefficient);
                // 更新股票持有数量
                if let Some(hold) = self.hold.get(&code) {
                    self.hold.insert(code, hold + amount);
                } else {
                    self.hold.insert(code, amount);
                }
            }
            OrderType::Sell => {
                // 更新资金剩余
                self.cash = self.cash
                    + price
                        * amount as f64
                        * (1.0 - self.commission_coefficient - self.tax_coefficient);
                // 更新股票持有数量
                if let Some(hold) = self.hold.get(&code) {
                    if amount == *hold {
                        self.hold.remove(&code);
                    } else {
                        self.hold.insert(code, hold - amount);
                    }
                }
            }
        }
    }
}

/// 模拟股票交易
/// codes: 股票代码, 数组，必填
/// init_cash: 初始现金，必填
/// commission_coeff: 佣金系数，选填
/// tax_coeff: 印花税系数，选填
pub async fn simulate_stock_trade(
    mut db: Connection<Db>,
    codes: Vec<String>,
    init_cash: f64,
    commission_coeff: Option<f64>,
    tax_coeff: Option<f64>,
    n1_range: Option<(usize, usize)>,
    n2_range: Option<(usize, usize)>,
    win_range: Option<(f64, f64)>,
    loss_range: Option<(f64, f64)>,
    adjust_range: Option<(usize, usize)>,
) -> HashMap<
    String,
    (
        (Vec<TradeResult>, Vec<OperateRecord>),
        (
            Option<usize>,
            Option<usize>,
            Option<f64>,
            Option<f64>,
            Option<usize>,
        ),
    ),
> {
    // 初始化持有股票数
    let init_hold: HashMap<String, usize> =
        codes.clone().into_iter().map(|code| (code, 0)).collect();
    // 初始化佣金系数和印花税系数
    let commission_coeff = commission_coeff.unwrap_or(DEFAULT_COMMISSION_COEFFICIENT);
    let tax_coeff = tax_coeff.unwrap_or(DEFAULT_TAX_COEFFICIENT);
    // 初始化股票账户
    let st_account: ST_Account = ST_Account::new(init_hold, init_cash, commission_coeff, tax_coeff);
    let mut code_map: HashMap<
        String,
        (
            (Vec<TradeResult>, Vec<OperateRecord>),
            (
                Option<usize>,
                Option<usize>,
                Option<f64>,
                Option<f64>,
                Option<usize>,
            ),
        ),
    > = HashMap::new();
    // 模拟交易
    for code in codes {
        let df_stock = get_local_stock_price_data(&mut db, code.clone())
            .await
            .expect("获取数据错误");
        let result: (
            (Vec<TradeResult>, Vec<OperateRecord>),
            (
                Option<usize>,
                Option<usize>,
                Option<f64>,
                Option<f64>,
                Option<usize>,
            ),
        ) = cal_ndayavg_mc(
            10000,
            st_account.clone(),
            df_stock,
            n1_range,
            n2_range,
            win_range,
            loss_range,
            adjust_range,
        );
        code_map.insert(code, result);
    }
    code_map
}
#[derive(Debug)]
struct TradeSignal {
    code: String,          // 股票代码
    date: Option<String>,  // 交易日期
    open: Option<f64>,     // 开盘价
    close: Option<f64>,    // 收盘价
    high: Option<f64>,     // 最高价
    low: Option<f64>,      // 最低价
    volume: Option<f64>,   // 成交量
    signal: Option<usize>, // 交易信号
    n1_high: Option<f64>,  // N1 日最高价
    n2_low: Option<f64>,   // N2 日最低价
    atr_14: Option<f64>,   // ATR 14 日
}

fn col_trade_signal(
    df_stock: &Vec<StockPriceInfo>,
    n1: Option<usize>,
    n2: Option<usize>,
    period: Option<usize>,
    n_win: Option<f64>,
    n_loss: Option<f64>,
) -> Vec<TradeSignal> {
    let mut result: Vec<TradeSignal> = df_stock
        .into_iter()
        .map(|stock| TradeSignal {
            code: stock.ts_code.clone(),
            date: stock.trade_date.clone(),
            open: stock.open,
            close: stock.close,
            high: stock.high,
            low: stock.low,
            volume: stock.vol,
            signal: None,
            n1_high: None,
            n2_low: None,
            atr_14: None,
        })
        .collect();

    let n1_high = n1.unwrap_or(14);
    let n2_low = n2.unwrap_or(2);
    let period = period.unwrap_or(14);
    let n_win = n_win.unwrap_or(2.0);
    let n_loss = n_loss.unwrap_or(0.8);
    // N1 日最高价
    let mut max = Maximum::new(n1_high).unwrap();
    // N2 日最低价
    let mut min = Minimum::new(n2_low).unwrap();

    for stock in result.iter_mut() {
        stock.n1_high = Some(max.next(stock.high.unwrap_or(0.0)));
        stock.n2_low = Some(min.next(stock.low.unwrap_or(0.0)));
    }
    let mut n1_queue: VecDeque<Option<f64>> = result.iter().map(|v| v.n1_high).collect();
    // 整体数据向右移动一位
    n1_queue.push_front(n1_queue.front().unwrap().clone());
    n1_queue.pop_back();
    for (index, today) in result.iter_mut().enumerate() {
        today.n1_high = n1_queue[index];
    }
    let mut n2_queue: VecDeque<Option<f64>> = result.iter().map(|v| v.n2_low).collect();
    // 整体数据向右移动一位
    n2_queue.push_front(n2_queue.front().unwrap().clone());
    n2_queue.pop_back();
    for (index, today) in result.iter_mut().enumerate() {
        today.n2_low = n2_queue[index];
    }
    // ATR 计算
    let mut atr = ATR::new(period).unwrap();
    // 创建了一个 result 的不可变引用, 后续又创建了一个可变引用, 这是不允许的
    // 因此使用 collect 方法将 result 转换为可变的 Vec, 消费掉 data 的所有权
    let data: Vec<DataItem> = result
        .iter()
        .map(|stock| {
            let res = match DataItem::builder()
                .open(stock.open.unwrap_or(0.0))
                .high(stock.high.unwrap_or(0.0))
                .low(stock.low.unwrap_or(0.0))
                .close(stock.close.unwrap_or(0.0))
                .volume(stock.volume.unwrap_or(0.0))
                .build()
            {
                Ok(res) => res,
                Err(e) => {
                    println!("{:#?}", e);
                    DataItem::builder().build().unwrap()
                }
            };
            res
        })
        .collect();

    // ATR 计算
    for (index, di) in data.into_iter().enumerate() {
        if index < result.len() - 1 {
            // atr 数据右移一位
            result[index + 1].atr_14 = Some(atr.next(&di));
        }
    }
    result[0].atr_14 = result[1].atr_14;
    // 所有右移操作都是为了让今天的信号根据昨天的数据进行计算
    // 买入价
    let mut buy_price = 0.0;
    for today in result.iter_mut() {
        // 今日收盘价大于 N1 日最高价
        if today.close.unwrap() > today.n1_high.unwrap() {
            buy_price = today.close.unwrap_or(0.0) as f64;
            today.signal = Some(1);
        } else if buy_price > 0.0
            && today.close.unwrap() < buy_price
            && (today.close.unwrap() < buy_price - today.atr_14.unwrap() * n_loss)
        {
            // 今日收盘价小于买入价且小买入价减去 N2 日最低价的 ATR 乘以 n_loss
            buy_price = 0.0;
            today.signal = Some(0);
        } else if buy_price > 0.0
            && today.close.unwrap() > buy_price
            && today.close.unwrap() > buy_price + today.atr_14.unwrap() * n_win
        {
            // 今日收盘价大于买入价且大于买入价加上 N1 日最高价的 ATR 乘以 n_win
            buy_price = 0.0;
            today.signal = Some(0);
        } else if today.close.unwrap() < today.n2_low.unwrap() {
            buy_price = 0.0;
            today.signal = Some(0);
        }
    }
    let mut signal_queue: VecDeque<Option<usize>> = result.iter().map(|v| v.signal).collect();
    // 整体数据向右移动一位
    signal_queue.push_front(None);
    signal_queue.pop_back();
    for (index, today) in result.iter_mut().enumerate() {
        today.signal = signal_queue[index];
    }
    result
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TradeResult {
    code: String,              // 股票代码
    date: Option<String>,      // 交易日期
    open: Option<f64>,         // 开盘价
    close: Option<f64>,        // 收盘价
    high: Option<f64>,         // 最高价
    low: Option<f64>,          // 最低价
    volume: Option<f64>,       // 成交量
    signal: Option<usize>,     // 交易信号
    n1_high: Option<f64>,      // N1 日最高价
    n2_low: Option<f64>,       // N2 日最低价
    atr_14: Option<f64>,       // ATR 14 日
    total_assets: Option<f64>, // 总资产
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct OperateRecord {
    order_type: OrderType,
    hold: usize,
    assets: f64,
    operate_num: usize,
    close: f64,
    operate_date: Option<String>,
}
/// adjust_hold: 动态持仓买入/卖出波动线
fn simulate_trade(
    df_stock: Vec<TradeSignal>,
    account: &mut ST_Account,
    adjust_hold: Option<usize>,
) -> (Vec<TradeResult>, Vec<OperateRecord>) {
    let mut has_buy = false;
    let mut operate_query: Vec<OperateRecord> = vec![];

    let mut df_stock: Vec<TradeResult> = df_stock
        .into_iter()
        .map(|stock| TradeResult {
            code: stock.code,
            date: stock.date,
            open: stock.open,
            close: stock.close,
            high: stock.high,
            low: stock.low,
            volume: stock.volume,
            signal: stock.signal,
            n1_high: stock.n1_high,
            n2_low: stock.n2_low,
            atr_14: stock.atr_14,
            total_assets: None,
        })
        .collect();
    for today in df_stock.iter_mut() {
        if today.signal.unwrap_or(0) == 1 && !has_buy {
            // 买入信号
            has_buy = true;
            account.send_order(
                today.code.clone(),
                (account.cash_available() * 0.01 / today.atr_14.unwrap_or(1.0)).floor() as usize,
                today.close.unwrap(),
                OrderType::Buy,
            );
            operate_query.push(OperateRecord {
                order_type: OrderType::Buy,
                hold: account.hold_available(today.code.clone()),
                assets: account.latest_assets(today.close.unwrap()),
                operate_num: (account.cash_available() * 0.01 / today.atr_14.unwrap_or(1.0)).floor()
                    as usize,
                operate_date: today.date.clone(),
                close: today.close.unwrap(),
            });
        } else if today.signal.unwrap_or(0) == 0 && has_buy {
            // 卖出信号
            has_buy = false;
            let operate_num = account.hold_available(today.code.clone());
            account.send_order(
                today.code.clone(),
                operate_num,
                today.close.unwrap(),
                OrderType::Sell,
            );
            operate_query.push(OperateRecord {
                order_type: OrderType::Sell,
                hold: account.hold_available(today.code.clone()),
                assets: account.latest_assets(today.close.unwrap()),
                operate_num,
                operate_date: today.date.clone(),
                close: today.close.unwrap(),
            });
        }
        // 动态计算持仓的股票数量
        if has_buy {
            let posit_num_wave = (account.latest_assets(today.close.unwrap()) * 0.01
                / today.atr_14.unwrap())
            .floor() as usize;
            // 波动后加仓
            if posit_num_wave
                > (account.hold_available(today.code.clone()) + adjust_hold.unwrap_or(0))
            {
                let operate_num = posit_num_wave - account.hold_available(today.code.clone());
                account.send_order(
                    today.code.clone(),
                    operate_num,
                    today.close.unwrap(),
                    OrderType::Buy,
                );
                operate_query.push(OperateRecord {
                    order_type: OrderType::Buy,
                    hold: account.hold_available(today.code.clone()),
                    assets: account.latest_assets(today.close.unwrap()),
                    operate_num,
                    operate_date: today.date.clone(),
                    close: today.close.unwrap(),
                });
            }
            // 波动后减仓
            if posit_num_wave
                < (account.hold_available(today.code.clone()) - adjust_hold.unwrap_or(0))
            {
                let operate_num = account.hold_available(today.code.clone()) - posit_num_wave;
                account.send_order(
                    today.code.clone(),
                    operate_num,
                    today.close.unwrap(),
                    OrderType::Sell,
                );
                operate_query.push(OperateRecord {
                    order_type: OrderType::Sell,
                    hold: account.hold_available(today.code.clone()),
                    assets: account.latest_assets(today.close.unwrap()),
                    operate_num,
                    operate_date: today.date.clone(),
                    close: today.close.unwrap(),
                });
            }
        }

        today.total_assets = Some(account.latest_assets(today.close.unwrap()));
    }
    (df_stock, operate_query)
}

// 蒙特卡洛算法模拟最优参数
fn cal_ndayavg_mc(
    n: usize,
    account: ST_Account,
    df_stock: Vec<StockPriceInfo>,
    n1_range: Option<(usize, usize)>,
    n2_range: Option<(usize, usize)>,
    win_range: Option<(f64, f64)>,
    loss_range: Option<(f64, f64)>,
    adjust_range: Option<(usize, usize)>,
) -> (
    (Vec<TradeResult>, Vec<OperateRecord>),
    (
        Option<usize>,
        Option<usize>,
        Option<f64>,
        Option<f64>,
        Option<usize>,
    ),
) {
    let (n1_min, n1_max) = n1_range.unwrap_or((5, 20));
    let (n2_min, n2_max) = n2_range.unwrap_or((1, 15));
    let (win_min, win_max) = win_range.unwrap_or((1.5, 2.5));
    let (loss_min, loss_max) = loss_range.unwrap_or((0.5, 1.5));
    let (adjust_min, adjust_max) = adjust_range.unwrap_or((0, 100));
    let mut max_total = 0.0;
    let mut best_param = (None, None, None, None, None);
    let mut simulate_result = (vec![], vec![]);
    for _ in 0..n {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        let n1 = rng.gen_range(n1_min..n1_max);
        let n2 = rng.gen_range(n2_min..n2_max);
        let win = rng.gen_range(win_min..win_max);
        let loss = rng.gen_range(loss_min..loss_max);
        let adjust = rng.gen_range(adjust_min..adjust_max);
        let mut new_account = account.clone();
        let new_df_stock = df_stock.clone();
        let df_stock_trade_signal =
            col_trade_signal(&new_df_stock, Some(n1), Some(n2), Some(14), None, None);
        let result = simulate_trade(df_stock_trade_signal, &mut new_account, Some(adjust));
        let total_assets = result.0.last().unwrap().total_assets.unwrap();
        if max_total < total_assets {
            max_total = total_assets;
            best_param = (Some(n1), Some(n2), Some(win), Some(loss), Some(adjust));
            simulate_result = result;
        }
    }
    (simulate_result, best_param)
}
