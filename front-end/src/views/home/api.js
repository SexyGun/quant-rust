import request from '@/utils/request';


export const testGet = () => request({ method: 'GET', url: '/data' });
export const testDataBase = () => request({ method: 'GET', url: '/diesel-async' });
export const testPost = (data) => request({ method: 'POST', url: '/diesel-async', data });
export const getPost = (params) => request({ method: 'GET', url: '/diesel-async/byId', params });
export const delPost = (params) => request({ method: 'GET', url: '/diesel-async/delete', params });
export const testGetStock = () => request({ method: 'GET', url: '/stock/basic' });
