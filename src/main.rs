use std::{thread, time};
use actix_web::{App, get, HttpResponse, HttpServer};
use serde::{ser, Serialize};

mod middleware;

#[derive(Serialize)]
pub struct WebApiResponse<T: ser::Serialize> {
    pub code: u32,
    pub data: Option<T>,
    pub error: Option<String>,
}

pub fn success<T: ser::Serialize>(r: Option<T>) -> HttpResponse {
    HttpResponse::Ok().json(WebApiResponse {
        code: 0,
        data: r,
        error: None,
    })
}

pub fn error(err: Option<String>) -> HttpResponse {
    HttpResponse::Ok().json(WebApiResponse::<String> {
        code: 1,
        data: None,
        error: err,
    })
}

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
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::timed::Timed)
            .wrap(middleware::auth::Auth)
            .service(hello1)
            .service(hello2)
    })
        .bind(("127.0.0.1", 8099))?
        .run()
        .await
}