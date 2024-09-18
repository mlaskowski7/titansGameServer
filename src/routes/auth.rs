use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use crate::models::auth::User;
use crate::services::auth::{get_user_by_id, login_user, obtain_all_users, obtain_user, register_user};
use crate::utils::jwt::{extract_jwt_token, generate_jwt_token, validate_jwt_token};

#[derive(Deserialize)]
pub struct AuthBody {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct TokenBody{
    token: String,
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

#[get("/api/auth/user/{username}")]
pub async fn get_user(username: web::Path<String>, pool: web::Data<MySqlPool>) -> impl Responder {
    match obtain_user(&username, &pool).await {
        Ok(Some(user)) => HttpResponse::Ok().json(UserResp::new(user)),
        Ok(None) => HttpResponse::NotFound().json("User not found"),
        Err(e) => {
            eprintln!("Error occurred while registering new user {:?}", e);
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}

#[post("/api/auth/login")]
pub async fn login(auth_body: web::Json<AuthBody>, pool: web::Data<MySqlPool>) -> impl Responder {
    match login_user(&auth_body.username, &auth_body.password, &pool).await{
        Ok(Some(user)) => {
            let token = generate_jwt_token(user.id);
            HttpResponse::Ok().json(AuthResponse::new(user, token))
        }
        Ok(None) => HttpResponse::NotFound().json("Username or password is incorrect"),
        Err(e) => {
            eprintln!("Error during login: {}", e);
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
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

#[get("/api/auth/checkToken")]
pub async fn check_token(req: HttpRequest, pool: web::Data<MySqlPool>) -> impl Responder {
    match extract_jwt_token(&req) {
        Some(token) => {
            match validate_jwt_token(&token) {
                Ok(claims) => {
                    let user_id = claims.sub;
                    println!("User ID: {}", user_id);
                    match get_user_by_id((&user_id).parse::<i64>().unwrap(), &pool).await {
                        Ok(Some(user)) => {
                            HttpResponse::Ok().json(AuthResponse::new(user, token))
                        }
                        Ok(None) => HttpResponse::NotFound().json("User not found"),
                        Err(e) => {
                            eprintln!("Error occurred while fetching user {:?}", e);
                            HttpResponse::InternalServerError().json(e.to_string())
                        }
                    }
                },
                Err(_) => HttpResponse::Unauthorized().json("Invalid token"),
            }
        }
        None => HttpResponse::Unauthorized().body("Unauthorized"),
    }
}



pub fn config_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
    cfg.service(check_token);
    cfg.service(get_all_users);
    cfg.service(get_user);
}