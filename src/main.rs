use std::{
    thread::{self},
    time::{self, Duration},
};

use actix_web::{get, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use log::info;
use simple_kafka::kafka_producer;

use crate::middleware::kafka::message_handler;
use crate::{api::success, middleware::kafka::TOPIC};

mod api;
mod config;
mod db;
mod log_config;
mod middleware;

#[get("/hello1")]
async fn hello1() -> HttpResponse {
    info!("hello1 start");

    let _x = tokio::spawn(async move { handle(1) });

    let _x = tokio::spawn(async move { handle(11) });

    let _x = tokio::spawn(handle_async(2));

    let _xx = tokio::spawn(handle_async(22));

    let _x = tokio::task::spawn_blocking(|| {
        handle(3);
    });

    let _x = tokio::task::spawn_blocking(|| {
        handle(33);
    });

    let _x = tokio::task::spawn_blocking(|| handle_async(4));

    let _x = tokio::task::spawn_blocking(|| {
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

#[get("/send")]
async fn send() -> HttpResponse {
    let _ = kafka_producer::send_timeout(TOPIC, "key", "hello".as_bytes(), Duration::from_secs(3))
        .await;
    success(Some(true))
}

#[get("/set_redis")]
async fn set_redis() -> HttpResponse{
    let result = middleware::redis_client::set_ex("hello".to_string(), "hello".to_string(), 100);
    success(Some(result))
}

#[get("/get_redis")]
async fn get_redis() -> HttpResponse{
    let result = middleware::redis_client::get("hello".to_string());
    success(Some(result))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    log_config::init_log();

    let database_url = config::SERVER_CONFIG.database_url();
    let (host, port) = config::SERVER_CONFIG.get_app_host_port();
    db::init_db(database_url).await;

    let kafka_config = &config::SERVER_CONFIG.kafka_config;
    let _init_task = tokio::spawn(async {
        let simple_kafka_config: simple_kafka::KafkaConfig = kafka_config.to_owned().into();
        simple_kafka::kafka_init::init_producers(&simple_kafka_config).await;
        simple_kafka::kafka_init::init_consumers(&simple_kafka_config, TOPIC, message_handler).await;
    });
    info!("app started http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::timed::Timed)
            .wrap(middleware::auth::Auth)
            .service(hello1)
            .service(hello2)
            .service(api::user::routes())
            .service(send)
            .service(set_redis)
            .service(get_redis)
    })
    .bind((host, port))?
    .run()
    .await
}
