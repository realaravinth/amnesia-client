use actix_web::{web, HttpResponse, Responder};
use awc::{Client, Connector};
use openssl::ssl::{SslConnector, SslMethod};
use std::sync::{Arc, RwLock};

use std::io::Cursor;

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

pub async fn upload() -> impl Responder {
    let payload = utils::prepare_payload();
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();
    let client = Client::build()
        .connector(Connector::new().ssl(builder.build()).finish())
        .finish();

    let mut form = crate::multipart::common::client::multipart::Form::default();
    let bytes = Cursor::new(payload.await.unwrap());
    let addr = "http://amnesic.herokuapp.com/archive/";
    form.add_reader_file("input", bytes, "/home/aravinth/yoyo");
    let response = client
        .post(addr)
        .content_type(form.content_type())
        .send_body(crate::multipart::actix::body::Body::from(form))
        .await
        .unwrap();
    HttpResponse::Ok()
}

pub async fn dump() -> impl Responder {
    let payload = utils::prepare_dump();
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();
    let client = Client::build()
        .connector(Connector::new().ssl(builder.build()).finish())
        .finish();
    let mut form = crate::multipart::common::client::multipart::Form::default();
    let bytes = Cursor::new(payload.await.unwrap());
    let addr = "http://amnesic.herokuapp.com/archive/";
    form.add_reader_file("input", bytes, "/home/aravinth/yoyo");
    let response = client
        .post(addr)
        .content_type(form.content_type())
        .send_body(crate::multipart::actix::body::Body::from(form))
        .await
        .unwrap();
    HttpResponse::Ok()
}

pub async fn clear() -> impl Responder {
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();
    let client = Client::build()
        .connector(Connector::new().ssl(builder.build()).finish())
        .finish();
    let payload = " <h1>Data unavailable</h1>";
    let mut form = crate::multipart::common::client::multipart::Form::default();
    let bytes = Cursor::new(payload);
    let addr = "http://amnesic.herokuapp.com/archive/";
    form.add_reader_file("input", bytes, "/home/aravinth/clear");
    let response = client
        .post(addr)
        .content_type(form.content_type())
        .send_body(crate::multipart::actix::body::Body::from(form))
        .await
        .unwrap();
    HttpResponse::Ok()
}
