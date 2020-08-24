extern crate pnet;
use pnet::datalink::{self, NetworkInterface};
use std::env;
use std::sync::mpsc;
use std::sync::{Arc, RwLock};
use std::thread;

mod handler;
mod server;
mod utils;

use crate::server::{server, State};
use crate::utils::{cleanup, setup};
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let setup_dir = setup();

    let interface_name = env::args().nth(1).unwrap();
    let interface_names_match = |iface: &NetworkInterface| iface.name == interface_name;
    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    {
        let _ = interfaces
            .into_iter()
            .filter(interface_names_match)
            .next()
            .unwrap();
    }
    let (tx, rx) = mpsc::channel();

    let data = Arc::new(RwLock::new(State::new(interface_name.to_string())));

    setup_dir.await.expect(
        "[Error]: Setup failed. Couldn't create dir at /tmp/. Do you have write permissions?",
    );
    thread::spawn(move || {
        let _ = server(data, tx);
    });
    let srv = rx.recv().unwrap();
    srv.await?;
    cleanup().await?;
    Ok(())
}
