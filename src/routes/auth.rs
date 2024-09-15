use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthBody {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct TokenBody {
    token: String,
}

#[post("/auth/login")]
pub fn login(auth_body: web::Json<AuthBody>) -> impl Responder {
    //TODO: login logic
    HttpResponse::Ok()
}

#[post("/auth/register")]
pub fn register(auth_body: web::Json<AuthBody>) -> impl Responder {
    //TODO: register logic
    HttpResponse::Ok()
}

#[post("/auth/checkToken")]
pub fn check_token(token: web::Json<TokenBody>) -> impl Responder {
    //TODO: check token logic
    HttpResponse::Ok()
}