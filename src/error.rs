use onionsalt::crypto::NaClError;
use std::error::Error as StdError;
use std::fmt;

use crate::api::models::ErrorResponse;

/// A `Result` alias where the `Err` case is `infisical_api::Error`.
pub type Result<T> = std::result::Result<T, Error>;

/// The Errors that may occur while utilizing infisical_api functionality
pub struct Error {
    inner: Box<Inner>,
}

pub(crate) type BoxError = Box<dyn StdError + Send + Sync>;

struct Inner {
    kind: Kind,
    source: Option<BoxError>,
}

impl Error {
    pub(crate) fn new<E>(kind: Kind, source: Option<E>) -> Error
    where
        E: Into<BoxError>,
    {
        Error {
            inner: Box::new(Inner {
                kind,
                source: source.map(Into::into),
            }),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = f.debug_struct("infisical_rs::Error");

        builder.field("kind", &self.inner.kind);

        if let Some(ref source) = self.inner.source {
            builder.field("source", source);
        }

        builder.finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.inner.kind {
            Kind::Encrypt => f.write_str("Encryption error")?,
            Kind::Decrypt => f.write_str("Decryption error")?,
            Kind::Reqwest => f.write_str("Reqwest error")?,
            Kind::UTF8 => f.write_str("UTF8 error")?,
            Kind::NaCl => f.write_str("NaCl error")?,
            Kind::Builder => f.write_str("Builder error")?,
            Kind::API => f.write_str("Infisical API error")?,
            Kind::MalformedServiceToken => f.write_str("Malformed Service Token error")?,
        };

        if let Some(e) = &self.inner.source {
            write!(f, ": {}", e);
        }

        Ok(())
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner.source.as_ref().map(|e| &**e as _)
    }
}

impl StdError for ErrorResponse {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Infisical API error");

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) enum Kind {
    Encrypt,
    Decrypt,
    Reqwest,
    UTF8,
    NaCl,
    Builder,
    API,
    MalformedServiceToken,
}

impl From<aes_gcm::Error> for Error {
    fn from(_err: aes_gcm::Error) -> Error {
        Error::new(Kind::Decrypt, None::<Error>)
    }
}

impl From<NaClError> for Error {
    fn from(err: NaClError) -> Error {
        nacl(err)
    }
}

// This just swallows any detail that InvalidLength would provide
// TODO: Find a better way to handle
impl From<aes_gcm::aes::cipher::InvalidLength> for Error {
    fn from(err: aes_gcm::aes::cipher::InvalidLength) -> Error {
        Error::new(Kind::Decrypt, None::<Error>)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        reqwest(err)
    }
}

impl From<ErrorResponse> for Error {
    fn from(err: ErrorResponse) -> Error {
        api(err)
    }
}

pub(crate) fn encrypt<E: Into<BoxError>>(e: E) -> Error {
    Error::new(Kind::Encrypt, Some(e))
}

pub(crate) fn decrypt<E: Into<BoxError>>(e: E) -> Error {
    Error::new(Kind::Decrypt, Some(e))
}

pub(crate) fn reqwest<E: Into<BoxError>>(e: E) -> Error {
    Error::new(Kind::Reqwest, Some(e))
}

pub(crate) fn utf8<E: Into<BoxError>>(e: E) -> Error {
    Error::new(Kind::UTF8, Some(e))
}

pub(crate) fn api<E: Into<BoxError>>(e: E) -> Error {
    Error::new(Kind::API, Some(e))
}

pub(crate) fn malformed_service_token() -> Error {
    Error::new(
        Kind::MalformedServiceToken,
        Some("The provided service token does not match the expected format"),
    )
}

pub(crate) fn nacl(e: NaClError) -> Error {
    match e {
        NaClError::IOError(io_error) => Error::new(Kind::NaCl, Some(io_error)),
        NaClError::WrongKey => Error::new(Kind::NaCl, None::<Error>),
        NaClError::RecvError(recv_error) => Error::new(Kind::NaCl, Some(recv_error)),
        NaClError::AuthFailed => Error::new(Kind::NaCl, None::<Error>),
    }
}

pub(crate) fn builder<E: Into<BoxError>>(e: E) -> Error {
    Error::new(Kind::Builder, Some(e))
}
