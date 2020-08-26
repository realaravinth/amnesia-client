// Copyright 2017 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::multipart::common;
use actix_http::error::ResponseError;
use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum Error {
    MultipartError(common::client::Error),
}

impl ResponseError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::MultipartError(ref e) => e.fmt(f),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::MultipartError(ref cause) => cause.description(),
        }
    }

    fn cause(&self) -> Option<&dyn StdError> {
        match *self {
            Error::MultipartError(ref e) => Some(e),
        }
    }
}
