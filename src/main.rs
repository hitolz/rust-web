use actix_web::{get, App, HttpResponse, HttpServer};
use log::info;
use std::{thread, time};

use crate::api::success;

mod api;
mod config;
mod db;
mod log_config;
mod middleware;

#[get("/hello1")]
async fn hello1() -> HttpResponse {
    thread::sleep(time::Duration::from_secs(1));
    success(Option::from(format!("Hello1")))
}

#[get("/hello2")]
async fn hello2() -> HttpResponse {
    thread::sleep(time::Duration::from_secs(1));
    success(Option::from(format!("Hello2")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = config::SERVER_CONFIG.database_url();
    let (host, port) = config::SERVER_CONFIG.get_app_host_port();
    db::init_db(database_url).await;

    info!("app started http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::timed::Timed)
            .wrap(middleware::auth::Auth)
            .service(hello1)
            .service(hello2)
            .service(api::user::routes())
    })
    .bind((host, port))?
    .run()
    .await
}
