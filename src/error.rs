//! Custom errors for this applicatoin.
//!
//! Map errors from libraries to Error.
//!
//! Define a reusable Result type.

use log::error;
use std::net::AddrParseError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug, Serialize)]
pub enum Error {
    #[error("Parse error: {0}.")]
    Parse(String),
}

// Log out errors
fn log_error(error: Error) -> Error {
    error!("{:?}", error);
    error
}

impl From<AddrParseError> for Error {
    fn from(error: AddrParseError) -> Self {
        log_error(Error::Parse(error.to_string()))
    }
}
