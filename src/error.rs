use std::error::Error as StdError;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

pub struct Error {
    inner: Box<Inner>,
}

pub(crate) type Source = Box<dyn StdError + Send + Sync>;

struct Inner {
    kind: Kind,
    source: Option<Source>,
}

#[derive(Debug)]
pub enum Kind {
    Builder,
    Decode,
    InvalidResponse,
    KeyNotFound,
    MissingIndex,
    MissingSessionFlag,
    Request,
    ServiceNotFound,
    UnexpectedResponse,
}

impl Error {
    pub(crate) fn new(kind: Kind) -> Error {
        Error {
            inner: Box::new(Inner { kind, source: None }),
        }
    }

    pub(crate) fn with<S: Into<Source>>(mut self, source: S) -> Error {
        self.inner.source = Some(source.into());
        self
    }

    #[allow(unused)]
    pub fn kind(&self) -> &Kind {
        &self.inner.kind
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = fmt.debug_struct("Error");
        builder.field("kind", &self.inner.kind);
        if let Some(ref source) = self.inner.source {
            builder.field("source", source);
        }
        builder.finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref source) = self.inner.source {
            write!(f, "{}: {}", self.description(), source)
        } else {
            f.write_str(self.description())
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self.inner.kind {
            Kind::Builder => "builder error",
            Kind::Decode => "decoding error",
            Kind::InvalidResponse => "invalid response from server",
            Kind::KeyNotFound => "key not found",
            Kind::MissingIndex => "missing index",
            Kind::MissingSessionFlag => "missing session flag",
            Kind::Request => "error sending request",
            Kind::ServiceNotFound => "service not found",
            Kind::UnexpectedResponse => "unexpected response from server",
        }
    }

    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner.source.as_ref().map(|e| &**e as _)
    }
}

// Helpers
#[allow(unused)]
pub(crate) fn builder<E: Into<Source>>(e: E) -> Error {
    Error::new(Kind::Builder).with(e)
}

#[allow(unused)]
pub(crate) fn decode<E: Into<Source>>(e: E) -> Error {
    Error::new(Kind::Decode).with(e)
}

#[allow(unused)]
pub(crate) fn invalid_response<E: Into<Source>>(e: E) -> Error {
    Error::new(Kind::InvalidResponse).with(e)
}

#[allow(unused)]
pub(crate) fn key_not_found<E: Into<Source>>(e: E) -> Error {
    Error::new(Kind::KeyNotFound).with(e)
}

#[allow(unused)]
pub(crate) fn missing_index() -> Error {
    Error::new(Kind::MissingIndex)
}

#[allow(unused)]
pub(crate) fn missing_session_flag() -> Error {
    Error::new(Kind::MissingSessionFlag)
}

#[allow(unused)]
pub(crate) fn request<E: Into<Source>>(e: E) -> Error {
    Error::new(Kind::Request).with(e)
}

#[allow(unused)]
pub(crate) fn service_not_found<E: Into<Source>>(e: E) -> Error {
    Error::new(Kind::ServiceNotFound).with(e)
}

#[allow(unused)]
pub(crate) fn unexpected_response<E: Into<Source>>(e: E) -> Error {
    Error::new(Kind::UnexpectedResponse).with(e)
}
