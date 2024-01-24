use std::future::{ready, Ready};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::{LocalBoxFuture};
use dotenv::dotenv;
use std::env;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use crate::models::jwt::Claims;
use chrono::Utc;
// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct JwtMiddleware;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareService { service }))
    }
}

pub struct JwtMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
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
        dotenv().ok();

        let mut auth_error = None;

        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer "){
                    let token = &auth_str["Bearer ".len()..];
                    let secret_key = env::var("SECRET_KEY").expect("key must be set.");

                    match decode::<Claims>(&token, &DecodingKey::from_secret(secret_key.as_ref()), &Validation::new(Algorithm::HS256)){
                        Ok(decoded_token) => {
                            let claims = decoded_token.claims;
                            let current_time = Utc::now().timestamp() as usize;
                            // let user_id = claims.sub;

                            if current_time > claims.exp {
                                println!("Token is expired.");
                                auth_error = Some("Token is expired.");
                            } else {
                                println!("Token is valid and not expired.");
                                // auth_error = Some("Token is valid and not expired.");
                            }
                        },
                        Err(err) => {
                            println!("Token is invalid: {:?}", err);
                            auth_error = Some("Token is invalid.");
                            // You can handle invalid token case here (e.g., return an error response)
                        }
                    };
                }
            }
        } else {
            println!("No Authorization Header found.");
            auth_error = Some("No Authorization Header found.");
        }

        if let Some(error) = auth_error {
            // If there is an auth error, return a future with an error response
            return Box::pin(async move { Err(actix_web::error::ErrorUnauthorized(error)) });
        } else {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        }
    }
}
