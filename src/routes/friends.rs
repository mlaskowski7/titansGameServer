use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::MySqlPool;
use crate::services::friends::{add_friend, remove_friend};

#[derive(Deserialize)]
pub struct FriendBody {
    user_id: i32,
    friend_id: i32,
}

#[post("/api/friends/add")]
pub async fn add_new_friend(body: web::Json<FriendBody>, pool: web::Data<MySqlPool>) -> impl Responder {
    match add_friend(body.user_id, body.friend_id, &pool).await {
        Ok(_) => HttpResponse::Ok().json(body.friend_id),
        Err(e) => {
            eprintln!("Error occurred while adding new friend to the user {:?}", e);
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}

#[post("/api/friends/remove")]
pub async fn remove_friend_from_user(body: web::Json<FriendBody>, pool: web::Data<MySqlPool>) -> impl Responder {
    match remove_friend(body.user_id, body.friend_id, &pool).await {
        Ok(_) => HttpResponse::Ok().json(body.friend_id.to_string()),
        Err(e) => {
            eprintln!("Error occurred while removing friend from the user {:?}", e);
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}

pub fn config_friends_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(add_new_friend);
    cfg.service(remove_friend_from_user);
}