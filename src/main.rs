use ::time::Instant;
use actix_web::{get, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use log::info;
use rayon::prelude::*;
use std::{
    sync::Mutex,
    thread::{self, ThreadId},
    time::{self, Duration},
};

use crate::api::success;

mod api;
mod config;
mod db;
mod log_config;
mod middleware;

#[get("/hello1")]
async fn hello1() -> HttpResponse {
    info!("hello1 start");

    thread::spawn(|| {
        handle();
    });
    info!("hello1 end");
    success(Option::from(format!("Hello1")))
}

fn handle() {
    info!("sum start");
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .build()
        .unwrap();
    let start = Instant::now();
    let vec: Vec<i32> = (0..=1000).collect();
    pool.install(|| {
        let sum: i32 = vec
            .par_iter()
            .map(|x| {
                thread::sleep(Duration::from_millis(5));
                x
            })
            .sum();
        let elapsed_time = start.elapsed();
        info!("0..=1000 sum = {},elapsed time {}", sum, elapsed_time);
    });
}

#[get("/hello2")]
async fn hello2() -> HttpResponse {
    thread::sleep(time::Duration::from_secs(1));
    success(Option::from(format!("Hello2")))
}

#[actix_web::main]
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
