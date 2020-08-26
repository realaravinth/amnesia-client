extern crate actix_multipart_rfc7578;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
extern crate actix;

extern crate regex;
#[macro_use]
extern crate lazy_static;

use pretty_env_logger::env_logger::{from_env, Env};

use std::env;
use std::sync::mpsc;
use std::sync::{Arc, RwLock};
use std::thread;

mod handler;
mod multipart;
mod server;
mod utils;

use crate::server::{server, State};
use crate::utils::{cleanup, setup};
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let setup_dir = setup();
    from_env(Env::default().default_filter_or("info")).init();

    let interface_name = env::args().nth(1).unwrap_or_else(|| {
        error!("Please enter a network interface");
        panic!();
    });
    info!("Setting up working directory at /tmp/amnesia");

    setup_dir.await.unwrap_or_else(|error| {
        error!("Error encountered while setting up working directory at /tmp/");
        error!("{}", error);
        panic!();
    });

    let data = Arc::new(RwLock::new(State::new(interface_name.to_string())));
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let _ = server(data, tx);
    });
    let srv = rx.recv().unwrap();
    srv.await?;
    info!("Shutting down server...");
    cleanup().await?;
    info!("Server shutdown");
    Ok(())
}
