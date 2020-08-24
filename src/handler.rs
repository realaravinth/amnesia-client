use actix_web::{web, HttpResponse, Responder};
use std::sync::{Arc, RwLock};

use crate::server::State;

pub async fn listen(data: web::Data<Arc<RwLock<State>>>) -> impl Responder {
    let mut guard = data.write().unwrap();
    if let Some(process) = &mut guard.pid {
        let command = format!(r#" kill {}"#, process);
        let _ = run_script::run_script!(command);
        guard.pid = None
    } else {
        let command = format!(
            r#" tshark -i {} -w /tmp/amnesia/amnesia.pcapng "#,
            guard.interface
        );
        let child = run_script::spawn_script!(command).unwrap();
        *&mut guard.pid = Some(child.id().to_string());
    }

    HttpResponse::Ok()
}

pub async fn upload() -> impl Responder {
    use crate::utils::upload;
    // TODO write upload functionality
    HttpResponse::Ok()
}
