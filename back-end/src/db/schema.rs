diesel::table! {
    stock_info_list (ts_code) {
        ts_code -> Varchar,  // 主键
        symbol -> Nullable<Text>,
        name -> Nullable<Text>,
        area -> Nullable<Text>,
        industry -> Nullable<Text>,
        cnspell -> Nullable<Text>,
        market -> Nullable<Text>,
        list_date -> Nullable<Text>,
        act_name -> Nullable<Text>,
        act_ent_type -> Nullable<Text>,
    }
}
diesel::table! {
    rps_values (ts_code, trade_date) {
        ts_code -> Varchar,             // 主键
        trade_date -> Nullable<Text>,   // 日期
        rps -> Nullable<Double>,        // rps 值
        increase -> Nullable<Double>,   // 指定时间涨幅
    }
}
diesel::joinable!(rps_values -> stock_info_list (ts_code));
diesel::allow_tables_to_appear_in_same_query!(stock_info_list, rps_values);

diesel::table! {
    stock_daily_info (ts_code) {
        ts_code -> Varchar,                     // 主键
        trade_date -> Nullable<Text>,           // 日期
        open -> Nullable<Double>,               // 开盘价
        close -> Nullable<Double>,              // 收盘价
        high -> Nullable<Double>,               // 最高价
        low -> Nullable<Double>,                // 最低价
        pre_close -> Nullable<Double>,          // 昨收价【除权价，前复权】
        vol -> Nullable<Double>,                // 成交量 （手）
        change -> Nullable<Double>,             // 涨跌额
        pct_chg -> Nullable<Double>,            // 涨跌幅 【基于除权后的昨收计算的涨跌幅：（今收-除权昨收）/除权昨收 】
        amount -> Nullable<Double>,             // 成交额 （千元）
    }
}
