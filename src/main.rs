/*
 * @Author: shiyun.ling shiyun.ling@flexiv.com
 * @Date: 2022-07-10 00:59:45
 * @LastEditors: shiyun.ling
 * @LastEditTime: 2022-07-18 15:38:00
 * @Description: file content
 */
/*
 * @Author: stephen.ling lingfromsh@163.com
 * @Date: 2022-07-10 00:59:45
 * @LastEditors: shiyun.ling
 * @LastEditTime: 2022-07-17 12:02:15
 * @Description: entrypoint of chameleon
 */
mod apis;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(apis::get_api_scope())
    })
    .bind("0.0.0.0:9050")?
    .run()
    .await
}
