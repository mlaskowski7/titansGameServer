use std::env;
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use dotenvy::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use crate::models::auth::User;
use crate::services::auth::{obtain_all_users, register_user};

#[derive(Deserialize)]
pub struct AuthBody {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserResp {
    id: i64,
    username: String,
    created_at: DateTime<Utc>,
}

impl UserResp {
    pub fn new(user: User) -> Self {
        UserResp {
            id : user.id,
            username : user.username,
            created_at : user.created_at.unwrap()
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct AuthResponse {
    user: UserResp,
    token: String,
}

impl AuthResponse {
    pub fn new(user: User, token: String) -> Self {
        AuthResponse {
            user : UserResp::new(user),
            token,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
    sub: String,
    exp: usize,
}

impl JwtClaims {
    pub fn new(sub: String, exp: usize) -> Self {
        JwtClaims {
            sub,
            exp
        }
    }
}

fn generate_jwt_token(user_id: i64) -> String {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(1))
        .unwrap()
        .timestamp() as usize;

    let claims = JwtClaims::new(user_id.to_string(), expiration);

    dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_ref())).expect("Failed to create JWT token")
}

#[post("/api/auth/login")]
pub async fn login(auth_body: web::Json<AuthBody>) -> impl Responder {
    //TODO: login logic
    HttpResponse::Ok()
}

#[post("/api/auth/register")]
pub async fn register(auth_body: web::Json<AuthBody>, pool: web::Data<MySqlPool>) -> impl Responder {
    match register_user(&auth_body.username, &auth_body.password , &pool).await {
        Ok(Some(user)) => {
            let token = generate_jwt_token(user.id);
            HttpResponse::Ok().json(AuthResponse::new(user, token))
        },
        Ok(None) => HttpResponse::BadRequest().body("Username is already taken"),
        Err(e) => {
            eprintln!("Error occurred while registering new user {:?}", e);
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}

// #[post("/api/auth/checkToken")]
// pub async fn check_token(token_body: web::Json<TokenBody>) -> impl Responder {
//     HttpResponse::Ok()
// }

#[get("/api/auth/users")]
pub async fn get_all_users(pool: web::Data<MySqlPool>) -> impl Responder {
    match obtain_all_users(&pool).await {
        Ok(users) => {
            let result: Vec<UserResp> = users
                .into_iter()
                .map(UserResp::new)
                .collect();
            HttpResponse::Ok().json(result)
        },
        Err(e) => {
            eprintln!("An error occurred while trying to fetch users list from db {}", e);
            HttpResponse::InternalServerError().json({
                format!("Error fetching users: {:?}", e)
            })
        }
    }
}

pub fn config_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
    // cfg.service(check_token);
    cfg.service(get_all_users);
}