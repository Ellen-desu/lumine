use http::{
    header::{InvalidHeaderName, InvalidHeaderValue},
    method::InvalidMethod,
    uri::InvalidUri,
};
use std::io;
use thiserror::Error;

/// Represents errors that can occur while handling an HTTP request.
///
/// This enum provides a unified error model for the entire request
/// lifecycle, including request parsing, routing, and response
/// construction.
///
/// # Design
///
/// `Error` acts as a thin abstraction over lower-level errors coming
/// from external libraries (such as `http` and `std::io`) while also
/// defining framework-specific error conditions.
///
/// Most variants either:
///
/// - Wrap an underlying error as the source, or
/// - Represent a well-defined failure in request parsing or validation.
///
/// This approach keeps error propagation simple and avoids leaking
/// implementation details into higher-level code.
///
/// # Error propagation
///
/// Errors of this type are typically returned through Lumine's
/// [`Result`] alias and may be handled by the server runtime to
/// generate appropriate HTTP error responses.
///
/// # Notes
///
/// - Not all variants are intended to be matched exhaustively by
///   application code.
/// - In most cases, treating this enum as an opaque error type is
///   sufficient.
#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP error")]
    HTTPError {
        #[source]
        source: http::Error,
    },

    #[error("Malformed request line")]
    InvalidRequestLine,

    #[error("Method not recognized")]
    InvalidMethod {
        #[source]
        source: InvalidMethod,
    },

    #[error("Failed to construct URI")]
    InvalidUri {
        #[source]
        source: InvalidUri,
    },

    #[error("Unsupported HTTP version: {0}")]
    InvalidVersion(String),

    #[error("Unrecognized header name")]
    InvalidHeaderName {
        #[source]
        source: InvalidHeaderName,
    },

    #[error("Header value doesn't match expected format")]
    InvalidHeaderValue {
        #[source]
        source: InvalidHeaderValue,
    },

    #[error("Invalid request body: {0}")]
    InvalidBody(&'static str),

    #[error("IO error")]
    IOError {
        #[source]
        source: io::Error,
    },
}

impl From<http::Error> for Error {
    fn from(value: http::Error) -> Self {
        Self::HTTPError { source: value }
    }
}

impl From<InvalidMethod> for Error {
    fn from(value: InvalidMethod) -> Self {
        Self::InvalidMethod { source: value }
    }
}

impl From<InvalidUri> for Error {
    fn from(value: InvalidUri) -> Self {
        Self::InvalidUri { source: value }
    }
}

impl From<InvalidHeaderName> for Error {
    fn from(value: InvalidHeaderName) -> Self {
        Self::InvalidHeaderName { source: value }
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(value: InvalidHeaderValue) -> Self {
        Self::InvalidHeaderValue { source: value }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IOError { source: value }
    }
}
