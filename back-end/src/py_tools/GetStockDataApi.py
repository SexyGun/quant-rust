import tushare as ts
my_ts_token = '5ad981f92afb58c9b91c87a22d8e03417e13a7af1cbe458aecf95164'
ts.set_token(my_ts_token)
pro = ts.pro_api()

def GetStockDataApi(*args, **kwargs):
    # 获取股票原始数据
    df_stockload = pro.daily(
        ts_code=args[0], start_date=args[1], end_date=args[2]
    )
    if df_stockload.empty:
        return []
    return df_stockload.to_dict(orient='records')