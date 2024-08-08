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