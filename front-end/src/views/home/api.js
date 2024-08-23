import request from '@/utils/request';

export const testGetStock = () => request({ method: 'GET', url: '/stock/basic' });
