// Copyright 2017 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

extern crate actix_multipart_rfc7578 as actix_multipart;

use actix_multipart::client::multipart;
use awc::Client;
use std::io::Cursor;

#[actix_rt::main]
async fn main() {
    let addr = "http://127.0.0.1:3000/archive/";
    let mut form = multipart::Form::default();
    let bytes = Cursor::new("yolo motherfucker");
    form.add_reader_file("input", bytes, "/home/aravinth/yoyo");

    let response = Client::default()
        .post(addr)
        .content_type(form.content_type())
        .send_body(multipart::Body::from(form))
        .await;

    if let Ok(_) = response {
        println!("done...");
    } else {
        eprintln!("an error occurred");
    }
}
