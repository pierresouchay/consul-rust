use std::fmt::{self, Display};
use std::result;

pub use failure::ResultExt;
use failure::{Backtrace, Context, Fail};

use reqwest;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

#[derive(Clone, Debug, Eq, PartialEq, Fail)]
pub enum ErrorKind {
    #[fail(display = "Failed to parse JSON")]
    InvalidJson,
    #[fail(display = "Session flag is require to acquire lock")]
    MissingSessionFlag,
    #[fail(display = "Utf8Error")]
    Utf8Error(#[cause] std::str::Utf8Error),
    #[fail(display = "IntError")]
    IntError(#[cause] std::num::ParseIntError),
    // NOTE: reqwest::Error does not implement PartialEq, so we will have to work around that for
    // now
    //#[fail(display = "ReqwestError")]
    //ReqwestError(#[cause] reqwest::Error),
    #[fail(display = "reqwest error: {}", _0)]
    Reqwest(String),
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
