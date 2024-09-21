use std::env;
use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use dotenvy::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use titans_game_server::routes::auth::{config_auth_routes};
use titans_game_server::routes::characters::config_characters_routes;
use titans_game_server::routes::friends::config_friends_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting the HTTP server on localhost:8080");

    // connect to db
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create pool");

    // run http server
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")  // Allow requests from your frontend
                    .allowed_methods(vec!["GET", "POST", "PUT"])     // Allow specific HTTP methods
                    .allowed_headers(vec![http::header::CONTENT_TYPE])
                    .allow_any_header()
                    .max_age(3600)
            )
            .app_data(web::Data::new(pool.clone()))
            .configure(config_auth_routes)
            .configure(config_characters_routes)
            .configure(config_friends_routes)
    })
    .bind("127.0.0.1:8080")?
    .run().await


}
