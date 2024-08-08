CREATE TABLE stock_info_list (
    ts_code CHAR(20) PRIMARY KEY,  -- 设定为主键，长度足够
    symbol VARCHAR(20),            -- 股票代码
    name VARCHAR(100),             -- 股票名称
    area VARCHAR(20),              -- 地域
    industry VARCHAR(100),         -- 所属行业
    cnspell VARCHAR(200),          -- 拼音缩写
    market VARCHAR(20),            -- 市场类型
    list_date VARCHAR(50),         -- 上市日期
    act_name VARCHAR(100),         -- 实控人名称，长度更大以适应实际数据
    act_ent_type VARCHAR(100)      -- 实控人企业性质
);