use actix_web::{web, HttpResponse, Responder};
use std::sync::{Arc, RwLock};

use crate::server::State;
use crate::utils;
pub async fn listen(data: web::Data<Arc<RwLock<State>>>) -> impl Responder {
    let mut guard = data.write().unwrap();
    if let Some(process) = &mut guard.tshark_child {
        process.kill().unwrap();
        guard.tshark_child = None;
        guard.extract_http_stream.spawn().unwrap();
    } else {
        guard.tshark_child = Some(guard.spawn_tshark.spawn().expect("Failed to start tshark"));
    }

    HttpResponse::Ok()
}

pub async fn upload(data: web::Data<Arc<RwLock<State>>>) -> impl Responder {
    utils::upload().await.unwrap();
    HttpResponse::Ok()
}
