use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    http::header::HeaderValue,
    Error,
};
use futures_util::future::LocalBoxFuture;
use log::error;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Auth;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleWare<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleWare { service }))
    }
}

pub struct AuthMiddleWare<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleWare<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path();
        let value = HeaderValue::from_str("").unwrap();
        let token: &HeaderValue = req.headers().get("token").unwrap_or(&value);
        if token.len() > 0 || req.path().to_string() == "/login" {
            let fut: <S as Service<ServiceRequest>>::Future = self.service.call(req);
            Box::pin(async move {
                let res = fut.await;
                res
            })
        } else {
            error!("request path is not authorized please login,path = {}", req.path());
            Box::pin(async move { Err(ErrorUnauthorized("PLEASE LOGIN")) })
        }
    }
}
