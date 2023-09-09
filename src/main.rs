mod handlers;
mod models;
mod utils;

use actix_web::{web, App, HttpServer};
use dashmap::DashMap;
use log::info;

use handlers::{get_map, get_balance, mutate_map, health_check};
use crate::models::app_state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    utils::initialize_logger();

    info!("Starting server at 127.0.0.1:8000");

    let shared_data = web::Data::new(AppState {
        merkle_map: DashMap::new(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .route("/map", web::get().to(get_map))
            .route("/balance/{name}", web::get().to(get_balance))
            .route("/mutate", web::post().to(mutate_map))
            .route("/health", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
