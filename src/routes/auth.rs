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
pub async fn login(auth_body: web::Json<AuthBody>) -> impl Responder {
    //TODO: login logic
    HttpResponse::Ok()
}

#[post("/auth/register")]
pub async fn register(auth_body: web::Json<AuthBody>) -> impl Responder {
    //TODO: register logic
    HttpResponse::Ok()
}

#[post("/auth/checkToken")]
pub async fn check_token(token_body: web::Json<TokenBody>) -> impl Responder {
    //TODO: check token logic
    HttpResponse::Ok()
}