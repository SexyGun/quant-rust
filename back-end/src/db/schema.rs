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
        ts_code -> Varchar,  // 主键
        trade_date -> Nullable<Text>, // 日期
        rps -> Nullable<Double>, // rps 值
        increase -> Nullable<Double>, // 指定时间涨幅
    }
}
diesel::joinable!(rps_values -> stock_info_list (ts_code));
diesel::allow_tables_to_appear_in_same_query!(stock_info_list, rps_values);