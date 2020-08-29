use actix_files::Files;
use actix_rt::System;
use actix_web::middleware::Logger;
use actix_web::{dev::Server, web, App, HttpServer};

use std::process::{Child, Command};
use std::sync::mpsc;
use std::sync::{Arc, RwLock};

use crate::handler::{clear, dump, listen, upload};

pub fn server(data: Arc<RwLock<State>>, tx: mpsc::Sender<Server>) -> std::io::Result<()> {
    let mut sys = System::new("amnesia-localhost");
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(data.clone())
            .service(
                web::scope("/api")
                    .route("/toggleListen", web::get().to(listen))
                    .route("/upload", web::get().to(upload))
                    .route("/dump", web::get().to(dump))
                    .route("/clear", web::get().to(clear)),
            )
            .service(Files::new("/", "/var/www/amnesia-client/static").index_file("index.html"))
    })
    .bind("0.0.0.0:7000")?
    .run();
    let _ = tx.send(server.clone());
    sys.block_on(server)
}

#[derive(Debug)]
pub struct State {
    pub spawn_tshark: Command,
    pub extract_http_stream: Command,
    pub tshark_child: Option<Child>,
}

impl State {
    pub fn new(interface: String) -> Self {
        let mut spawn_tshark = Command::new("tshark");
        spawn_tshark.args(&[
            "-i",
            &interface,
            "-w",
            "/tmp/amnesia/capture/amnesia.pcapng",
        ]);
        let mut extract_http_stream = Command::new("tshark");
        extract_http_stream.args(&[
            "--export-objects",
            r#"http,/tmp/amnesia/http-stream"#,
            "-r",
            "/tmp/amnesia/capture/amnesia.pcapng",
        ]);
        let tshark_child: Option<Child> = None;
        State {
            spawn_tshark,
            tshark_child,
            extract_http_stream,
        }
    }
}
