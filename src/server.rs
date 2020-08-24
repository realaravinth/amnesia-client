use actix_files::Files;
use actix_rt::System;
use actix_web::middleware::Logger;
use actix_web::{dev::Server, web, App, HttpServer};

use std::process::{Child, Command};
use std::sync::mpsc;
use std::sync::{Arc, RwLock};

use crate::handler::{listen, upload};

pub fn server(data: Arc<RwLock<State>>, tx: mpsc::Sender<Server>) -> std::io::Result<()> {
    let mut sys = System::new("amnesia-localhost");
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(data.clone())
            .service(
                web::scope("/api")
                    .route("/toggleListen", web::get().to(listen))
                    .route("/upload", web::get().to(upload)),
            )
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("0.0.0.0:8000")?
    .run();
    let _ = tx.send(server.clone());
    sys.block_on(server)
}

#[derive(Debug)]
pub struct State {
    pub spawn_tshark: Command,
    pub tshark_child: Option<Child>,
}

impl State {
    pub fn new(interface: String) -> Self {
        let mut spawn_tshark = Command::new("tshark");
        spawn_tshark
            .arg("-i")
            .arg(interface)
            .arg("-w")
            .arg("/tmp/amnesia/amnesia.pcapng");
        let tshark_child: Option<Child> = None;
        State {
            spawn_tshark,
            tshark_child,
        }
    }
}