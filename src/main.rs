use std::{
    thread::{self},
    time::{self, Duration},
};

use ::time::Instant;
use actix_web::{get, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use log::info;
use rayon::prelude::*;

use crate::api::success;

mod api;
mod config;
mod db;
mod log_config;
mod middleware;

#[get("/hello1")]
async fn hello1() -> HttpResponse {
    info!("hello1 start");

    let x = tokio::spawn(async move {
        handle(1)
    });

    let x = tokio::spawn(async move {
        handle(11)
    });

    let x = tokio::spawn({
        handle_async(2)
    });

    let x = tokio::spawn({
        handle_async(22)
    });


    let x = tokio::task::spawn_blocking(||{
        handle(3);
    });

    let x = tokio::task::spawn_blocking(||{
        handle(33);
    });

    let x = tokio::task::spawn_blocking(||{
        handle_async(4)
    });

    let x = tokio::task::spawn_blocking(||{
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(handle_async(44))
    });

    info!("hello1 end");
    success(Option::from(format!("Hello1")))
}

async fn handle_async(x: i32) {
    info!("handle start x = {} ...", x);
    thread::sleep(Duration::from_secs(3));
}

fn handle(x: i32) {
    info!("handle start x = {} ...", x);
    thread::sleep(Duration::from_secs(3));
}

#[get("/hello2")]
async fn hello2() -> HttpResponse {
    thread::sleep(time::Duration::from_secs(1));
    success(Option::from(format!("Hello2")))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    log_config::init_log();

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
