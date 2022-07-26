/*
 * @Author: shiyun.ling shiyun.ling@flexiv.com
 * @Date: 2022-07-16 00:34:48
 * @LastEditors: shiyun.ling
 * @LastEditTime: 2022-07-16 00:34:51
 * @Description: api test
 */
import http from 'k6/http';

export default function () {
  http.get('http://localhost:9050/health');
}
