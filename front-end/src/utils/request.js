import axios from "axios";

// 配置基本的 Axios 实例
const axiosInstance = axios.create({
  baseURL: import.meta.env.BASE_URL, // 替换为你实际的 baseURL
});

/**
 * 发送 HTTP 请求
 * @param {string} method - 请求方法 (GET 或 POST)
 * @param {string} url - 请求 URL
 * @param {Object} [params] - 请求参数 (用于 GET 请求)
 * @param {Object} [data] - 请求数据 (用于 POST 请求)
 * @param {Object} [headers] - 请求头
 * @returns {Promise} - 请求的 Promise 对象
 */
async function request({ method, url, params = {}, data = {}, headers = {} }) {
  try {
    const config = {
      method,
      url,
      headers,
    };

    if (method.toUpperCase() === 'GET') {
      config.params = params;
    } else if (method.toUpperCase() === 'POST') {
      config.data = data;
    }
    const data = await axiosInstance(config);
    return data;
  } catch (error) {
    // 处理错误
    console.error('Request failed:', error.message);
    throw error; // 重新抛出错误以供调用者处理
  }
}

// 导出 request 方法
export default request;
// 示例用法

// GET 请求
// request('GET', '/data', { key: 'value' })
//   .then(data => console.log('GET response:', data))
//   .catch(error => console.error('GET error:', error));

// POST 请求
// request('POST', '/data', {}, { key: 'value' })
//   .then(data => console.log('POST response:', data))
//   .catch(error => console.error('POST error:', error));