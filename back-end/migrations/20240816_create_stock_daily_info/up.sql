CREATE TABLE stock_daily_info (
    ts_code CHAR(20),             -- 股票代码
    trade_date CHAR(8),           -- 交易日期
    open DOUBLE,                  -- 开盘价，可以为空
    close DOUBLE,                 -- 收盘价，可以为空
    high DOUBLE,                  -- 最高价，可以为空
    low DOUBLE,                   -- 最低价，可以为空
    pre_close DOUBLE NULL,        -- 昨收价（除权价，前复权），可以为空
    vol DOUBLE,                   -- 成交量（手），可以为空
    `change` DOUBLE NULL,           -- 涨跌额，可以为空
    pct_chg DOUBLE NULL,          -- 涨跌幅，可以为空（基于除权后的昨收计算的涨跌幅）
    amount DOUBLE NULL,           -- 成交额（千元），可以为空
    PRIMARY KEY (ts_code, trade_date) -- 复合主键
);