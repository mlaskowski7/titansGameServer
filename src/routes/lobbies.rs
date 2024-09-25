use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::MySqlPool;
use crate::services::lobbies::{add_new_lobby, update_user_lobby};

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
    match add_new_lobby(&body.name, body.state, body.max_players, &pool).await {
        Ok(lobby) => HttpResponse::Ok().json(lobby),
        Err(e) => {
            eprintln!("An error occurred while trying to add lobby to db {}", e);
            HttpResponse::InternalServerError().json({
                format!("Error adding lobby: {:?}", e)
            })
        }
    }
}

#[post("/api/lobbies/add")]
pub async fn add_user_to_lobby(body: web::Json<AddToLobbyBody>, pool: web::Data<MySqlPool>) -> impl Responder {
    match update_user_lobby(body.user_id, body.lobby_id, &pool).await {
        Ok(_) => HttpResponse::Ok().json(body.lobby_id),
        Err(e) => {
            eprintln!("Error occurred while adding user to lobby {:?}", e);
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}

pub fn config_lobbies_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_lobby);
    cfg.service(add_user_to_lobby);
}