CREATE TABLE rps_values (
    ts_code CHAR(20),                  -- 股票代码
    trade_date CHAR(8),                -- 交易日期
    rps DECIMAL(10, 2),                -- RPS 值，使用 DECIMAL 类型存储
    increase DECIMAL(10, 2),           -- 涨幅，使用 DECIMAL 类型存储
    PRIMARY KEY (ts_code, trade_date), -- 复合主键
    FOREIGN KEY (ts_code) REFERENCES stock_info_list(ts_code) -- 外键约束
);