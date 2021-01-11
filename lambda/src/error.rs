//! Provides error handling for this crate.

use std::error;
use std::fmt;
use crate::types::Diagnostic;

/// Default Error representation.
#[derive(Debug)]
pub struct Error {
    error_message: String,
    error_type: String
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = format!("{}: {}", self.error_type, self.error_message);
        f.write_str(&msg)
    }
}

impl Into<Diagnostic> for Error {
    fn into(self) -> Diagnostic {
        Diagnostic {
            error_message: self.error_message,
            error_type: self.error_type
        }
    }
}

impl From<hyper::error::Error> for Error {
    fn from(cause: hyper::error::Error) -> Self { cause.into_error() }
}

impl From<serde_json::Error> for Error {
    fn from(cause: serde_json::Error) -> Self { cause.into_error() }
}

impl From<std::env::VarError> for Error {
    fn from(cause: std::env::VarError) -> Self { cause.into_error() }
}

impl From<std::num::ParseIntError> for Error {
    fn from(cause: std::num::ParseIntError) -> Self { cause.into_error() }
}

impl From<http::Error> for Error {
    fn from(cause: http::Error) -> Self { cause.into_error() }
}

/// Converts external objects into Error.
pub trait IntoError {
    fn into_error(self) -> Error;
}

impl<T> IntoError for T where T: error::Error {
    fn into_error(self) -> Error {
        Error {
            error_message: format!("{}", self),
            error_type: type_name_of_val(self).to_owned(),
        }
    }
}

fn type_name_of_val<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}
