/*
 * @Author: stephen.ling lingfromsh@163.com
 * @Date: 2022-07-10 00:59:45
 * @LastEditors: stephen.ling
 * @LastEditTime: 2022-07-10 13:48:20
 * @Description: entrypoint of chameleon
 */
mod settings;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use settings::Settings;

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::new();

    println!("Settings: \n{:?}", settings);
    HttpServer::new(|| App::new().service(health_check))
        .bind("0.0.0.0:9050")?
        .run()
        .await
}
