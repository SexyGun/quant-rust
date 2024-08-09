import request from '@/utils/request';

export const getBasicQuery = (data) => request({ method: 'POST', url: '/stock/query', data });
