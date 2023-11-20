use actix_web::{get, App, HttpResponse, HttpServer};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::{thread, time};

use crate::api::success;

mod api;
mod db;
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
    let database_url = "mysql://root:12345678@localhost:3306/rust_web".to_string();
    db::init_db(database_url).await;

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::timed::Timed)
            .wrap(middleware::auth::Auth)
            .service(hello1)
            .service(hello2)
            .service(api::user::routes())
    })
    .bind(("127.0.0.1", 8099))?
    .run()
    .await
}
