use actix_web::{web, HttpResponse, Responder};
use crate::models::app_state::AppState;
use log::{info, warn};

pub async fn get_balance(data: web::Data<AppState>, name: web::Path<String>) -> impl Responder {
    let name_str = name.into_inner();

    match data.merkle_map.get(&name_str) {
        Some(balance_ref) => {
            let balance = *balance_ref.value();
            info!("Fetching balance for {}: {}", &name_str, balance);
            HttpResponse::Ok().json(balance)
        },
        None => {
            warn!("Name {} not found.", &name_str);
            HttpResponse::NotFound().body("Name not found.")
        },
    }
}
