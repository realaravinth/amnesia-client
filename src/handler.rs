use actix_web::{web, HttpResponse, Responder};
use std::sync::{Arc, RwLock};

use crate::server::State;

pub async fn listen(data: web::Data<Arc<RwLock<State>>>) -> impl Responder {
    let mut guard = data.write().unwrap();
    if let Some(process) = &mut guard.tshark_child {
        //        let _kill_tshark = uu
        process.kill().unwrap();
        guard.tshark_child = None
    } else {
        guard.tshark_child = Some(guard.spawn_tshark.spawn().expect("Failed to start tshark"));
    }

    HttpResponse::Ok()
}

pub async fn upload() -> impl Responder {
    use crate::utils::upload;
    // TODO write upload functionality
    HttpResponse::Ok()
}
