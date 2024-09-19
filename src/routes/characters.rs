use actix_web::{get, web, HttpResponse, Responder};
use sqlx::MySqlPool;
use crate::services::characters::obtain_all_characters;

#[get("/api/characters")]
pub async fn get_all_characters(pool: web::Data<MySqlPool>) -> impl Responder {
    match obtain_all_characters(&pool).await {
        Ok(characters) => {
            HttpResponse::Ok().json(characters)
        }
        Err(e) => {
            eprintln!("An error occurred while trying to fetch characters list from db {}", e);
            HttpResponse::InternalServerError().json({
                format!("Error fetching characters: {:?}", e)
            })
        }
    }
}

pub fn config_characters_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_characters);
}