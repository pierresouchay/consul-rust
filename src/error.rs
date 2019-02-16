use std::fmt::{self, Display};
use std::result;

pub use failure::ResultExt;
use failure::{Backtrace, Context, Fail};

use reqwest;
use serde_json;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

#[derive(Clone, Debug, Eq, PartialEq, Fail)]
pub enum ErrorKind {
    #[fail(display = "Failed to parse option value")]
    InvalidOption,
    #[fail(display = "Failed to parse index")]
    InvalidIndex,
    #[fail(display = "Failed to parse response")]
    InvalidResponse,
    #[fail(display = "Session flag is require to acquire lock")]
    MissingSessionFlag,
    #[fail(display = "Index missing from response header")]
    MissingIndex,
    #[fail(display = "Key not found")]
    KeyNotFound,
    #[fail(display = "Unexpected response: {}", _0)]
    UnexpectedResponse(String),
    #[fail(display = "Utf8Error")]
    Utf8Error(#[cause] std::str::Utf8Error),
    #[fail(display = "IntError")]
    IntError(#[cause] std::num::ParseIntError),
    #[fail(display = "Reqwest error: {}", _0)]
    Reqwest(String),
    #[fail(display = "Serde JSON error: {}", _0)]
    SerdeJson(String),
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error {
            inner: Context::new(ErrorKind::Reqwest(err.to_string())),
        }
    }
}

impl From<reqwest::UrlError> for Error {
    fn from(err: reqwest::UrlError) -> Error {
        Error {
            inner: Context::new(ErrorKind::Reqwest(err.to_string())),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error {
            inner: Context::new(ErrorKind::SerdeJson(err.to_string())),
        }
    }
}
