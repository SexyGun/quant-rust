import request from '@/utils/request';

export const getStockRpsTest = () => request({ method: 'GET', url: '/stock/test' });

export const getStockRpsList = (data) => request({ method: 'POST', url: '/stock/rps-top', data });
