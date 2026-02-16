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
    Http {
        #[source]
        source: http::Error,
    },
    #[error("IO error")]
    Io {
        #[source]
        source: std::io::Error,
    },
    #[error("failed to parse HTTP request")]
    Parser,
}

impl From<http::Error> for Error {
    fn from(value: http::Error) -> Self {
        Self::Http { source: value }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io { source: value }
    }
}

impl From<InvalidHeaderName> for Error {
    fn from(_: InvalidHeaderName) -> Self {
        Self::Parser
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(_: InvalidHeaderValue) -> Self {
        Self::Parser
    }
}

impl From<InvalidMethod> for Error {
    fn from(_: InvalidMethod) -> Self {
        Self::Parser
    }
}

impl From<InvalidUri> for Error {
    fn from(_: InvalidUri) -> Self {
        Self::Parser
    }
}
