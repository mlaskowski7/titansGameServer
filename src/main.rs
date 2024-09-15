use actix_web::{App, HttpServer};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Starting the HTTP server on localhost:8080");

    // run http server
    HttpServer::new(|| {
        App::new()
    })
    .bind("127.0.0.1:8080")?
    .run().await


}
