use actix_web::{post, web, Responder};
use serde::Deserialize;
use sqlx::MySqlPool;
use crate::routes::friends::{add_new_friend, remove_friend_from_user};

#[derive(Deserialize)]
pub struct LobbyBody {
    pub name: String,
    pub state: i32,
    pub max_players: i32
}

#[derive(Deserialize)]
pub struct AddToLobbyBody {
    pub user_id: i32,
    pub lobby_id: i32,
}

#[post("/api/lobbies")]
pub async fn create_lobby(body: web::Json<LobbyBody>, pool: web::Data<MySqlPool>) -> impl Responder {
    // TODO implement this
}

#[post("/api/lobbies/add")]
pub async fn add_to_lobby(body: web::Json<AddToLobbyBody>, pool: web::Data<MySqlPool>) -> impl Responder {
    // TODO implement this
}

pub fn config_lobbies_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_lobby);
    cfg.service(add_to_lobby);
}