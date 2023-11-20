use actix_web::HttpResponse;
use serde::{ser, Serialize};

pub mod user;




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