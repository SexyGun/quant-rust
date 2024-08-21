import request from '@/utils/request';

export const test_api = (data) => request({ method: 'POST', url: '/stock/simulate', data });

