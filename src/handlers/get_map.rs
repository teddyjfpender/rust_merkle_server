use std::collections::HashMap;

use crate::models::app_state::AppState;
use actix_web::{web, HttpResponse, Responder};
use log::info;

pub async fn get_map(data: web::Data<AppState>) -> impl Responder {
    info!("Fetching the entire map");
    let map: HashMap<String, u32> = data.merkle_map.iter().map(|ref_multi| (ref_multi.key().clone(), *ref_multi.value())).collect();
    HttpResponse::Ok().json(map)
}
