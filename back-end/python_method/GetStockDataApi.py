import tushare as ts
import pandas as pd

my_ts_token = "5ad981f92afb58c9b91c87a22d8e03417e13a7af1cbe458aecf95164"
ts.set_token(my_ts_token)
pro = ts.pro_api()


# def GetStockDataApi(StockCode, StockTimeS, StockTimeE):
#     """获取股票交易数据接口"""
#     # 获取股票原始数据
#     df_stockload = pro.daily(
#         ts_code=StockCode, start_date=StockTimeS, end_date=StockTimeE
#     )
#     # 将交易日期 str 格式转换为 datetime 格式
#     df_stockload["trade_date"] = pd.to_datetime(df_stockload["trade_date"])
#     # 将索引设置为交易日起，且 tushare 获取的数据为时间降，因此需要重新排序
#     stock_data = df_stockload.set_index("trade_date").sort_index()
#     return stock_data


def GetAllStockData():
    """取基础信息数据的备用列表，包括股票代码、名称、上市日期、退市日期等"""
    stock_list = pro.bak_basic()
    return stock_list
