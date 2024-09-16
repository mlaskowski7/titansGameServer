use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AuthBody {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct TokenBody {
    token: String,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseMessage {
    status: String,
    message: String,
}

#[post("/api/auth/login")]
pub async fn login(auth_body: web::Json<AuthBody>) -> impl Responder {
    //TODO: login logic
    HttpResponse::Ok()
}

#[post("/api/auth/register")]
pub async fn register(auth_body: web::Json<AuthBody>) -> impl Responder {
    //TODO: register logic
    HttpResponse::Ok();
}

#[post("/api/auth/checkToken")]
pub async fn check_token(token_body: web::Json<TokenBody>) -> impl Responder {
    //TODO: check token logic
    HttpResponse::Ok()
}

pub fn config_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
    cfg.service(check_token);
}