use actix_web::{get, post, put, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::MySqlPool;
use crate::models::lobbies::LobbyState;
use crate::services::lobbies::{add_new_lobby, exit_user_from_lobby, get_lobby_by_id, obtain_all_lobbies, obtain_lobby, set_next_lobby_state, update_user_lobby};

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

#[derive(Deserialize)]
struct NextStateBody {
    pub lobby_id: i32,
    pub lobby_state: String,
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

#[get("/api/lobbies")]
pub async fn get_all_lobbies(pool: web::Data<MySqlPool>) -> impl Responder {
    match obtain_all_lobbies(&pool).await {
        Ok(lobbies) => HttpResponse::Ok().json(lobbies),
        Err(e) => {
            eprintln!("An error occurred while trying to retrieve all lobbies from db {}", e);
            HttpResponse::InternalServerError().json({
                format!("Error retrieving lobbies: {:?}", e)
            })
        }
    }
}

#[get("/api/lobbies/{name}")]
pub async fn get_lobby_by_name(name: web::Path<String>, pool: web::Data<MySqlPool>) -> impl Responder {
    match obtain_lobby(name.into_inner(), &pool).await {
        Ok(lobby) => HttpResponse::Ok().json(lobby),
        Err(e) => {
            eprintln!("An error occurred while trying to retrieve lobby from db {}", e);
            HttpResponse::InternalServerError().json({
                format!("Error retrieving lobby: {:?}", e)
            })
        }
    }
}

#[get("/api/lobbies/id/{id}")]
pub async fn get_lobby(id: web::Path<i32>, pool: web::Data<MySqlPool>) -> impl Responder {
    match get_lobby_by_id(id.into_inner(), &pool).await {
        Ok(lobby) => HttpResponse::Ok().json(lobby),
        Err(e) => {
            eprintln!("An error occurred while trying to retrieve lobby from db {}", e);
            HttpResponse::InternalServerError().json({
                format!("Error retrieving lobby: {:?}", e)
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

#[post("/api/lobbies/exit/{user_id}")]
pub async fn exit_lobby(user_id: web::Path<String>, pool: web::Data<MySqlPool>) -> impl Responder {
    match exit_user_from_lobby(&user_id.parse::<i32>().unwrap(), &pool).await {
        Ok(_) => HttpResponse::Ok().json(user_id.parse::<i32>().unwrap()),
        Err(e) => {
            eprintln!("Error occurred while adding user to lobby {:?}", e);
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}

#[put("/api/lobbies/nextState")]
pub async fn next_lobby_state(body: web::Json<NextStateBody>, pool: web::Data<MySqlPool>) -> impl Responder {
    let state = LobbyState::from_str(&body.lobby_state);
    match set_next_lobby_state(&body.lobby_id, &state, &pool).await {
        Ok(_) => HttpResponse::Ok().json("next state uploaded successfully"),
        Err(e) => {
            eprintln!("Error occurred while setting next lobby state {:?}", e);
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}

pub fn config_lobbies_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_lobby);
    cfg.service(add_user_to_lobby);
    cfg.service(get_all_lobbies);
    cfg.service(get_lobby_by_name);
    cfg.service(exit_lobby);
    cfg.service(get_lobby);
    cfg.service(next_lobby_state);
}