/*
 * @Author: shiyun.ling shiyun.ling@flexiv.com
 * @Date: 2022-07-17 01:05:48
 * @LastEditors: shiyun.ling
 * @LastEditTime: 2022-07-17 12:01:55
 * @Description: file content
 */

pub mod image;

use actix_web::{web, Scope};

pub fn get_api_scope() -> Scope {
    web::scope("/image")
        .service(image::thumbnail)
}
