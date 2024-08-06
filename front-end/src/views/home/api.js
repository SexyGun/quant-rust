import request from '@/utils/request';


export const testGet = () => request({ method: 'GET', url: '/data' });