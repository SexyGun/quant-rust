import request from '@/utils/request';

export const getStockRps = (data) => request({ method: 'POST', url: '/stock/fetch_stock_rps_list', data });

export const getStockDaily = (data) => request({ method: 'POST', url: '/stock/fetch_stock_daily_range', data });

export const getStockRpsList = (data) => request({ method: 'POST', url: '/stock/rps-top', data });

export const clearStockRps = () => request({ method: 'GET', url: '/stock/clear/rps-top' });