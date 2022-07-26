/*
 * @Author: shiyun.ling shiyun.ling@flexiv.com
 * @Date: 202&2-07-17 01:06:03
 * @LastEditors: shiyun.ling
 * @LastEditTime: 2022-07-18 18:10:26
 * @Description: file content
 */

use actix_files::NamedFile;
use actix_web::{get, web, HttpResponse, Either};
use image::io::Reader as ImageReader;
use serde::Deserialize;
use std::io::Cursor;

#[derive(Deserialize)]
pub struct OperationParams {
    source: Option<String>,
    height: u32,
    width: u32,
}

type RegisterResult = Either<HttpResponse, NamedFile>;

#[get("/thumbnail")]
pub async fn thumbnail(operation_params: web::Query<OperationParams>) -> RegisterResult {
    match &operation_params.source {
        Some(url) => {
            if url.len() == 0 {
                Either::Left(HttpResponse::Ok().body("Missing required param: source"))
            } else {
                // download file to disk
                let resp = reqwest::get(url)
                    .await
                    .expect(format!("failed to download {}", url).as_str())
                    .bytes()
                    .await
                    .expect("invalid image source");

                let _thumbnail = ImageReader::new(Cursor::new(resp))
                    .with_guessed_format()
                    .expect("unknown image format")
                    .decode()
                    .expect("invalid image")
                    .thumbnail(operation_params.width, operation_params.height)
                    .save("tmpdata/images/temp.jpeg");

                Either::Right(NamedFile::open("tmpdata/images/temp.jpeg").expect("failed to open image"))
            }
        }
        None => Either::Left(HttpResponse::Ok().body("Missing required param: source")),
    }
}
