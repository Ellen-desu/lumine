//! Error handling definitions for the Lumine framework.
//!
//! This module defines the [`Error`] enum, which encapsulates various issues
//! that might arise while processing HTTP requests, including parsing
//! failures and constraint violations.

use http::{
    header::{InvalidHeaderName, InvalidHeaderValue},
    method::InvalidMethod,
    uri::InvalidUri,
};

/// Represents errors that can occur during the processing of a request.
#[derive(Debug)]
pub enum Error {
    /// The URI exceeded the allowed size limit.
    UriTooLarge,
    /// The request body exceeded the allowed size limit.
    BodyTooLarge,
    /// The request headers exceeded the allowed size limit.
    HeadersTooLarge,
    /// The query string exceeded the allowed size limit.
    QueryTooLarge,
    /// The requested HTTP version is not supported by Lumine.
    HttpVersionNotSupported,
    /// The request line (method, URI, version) is malformed.
    InvalidRequestLine,
    /// The request headers are malformed.
    InvalidHeaders,
    /// The requested feature or operation has not been implemented yet.
    Unimplemented,
}

impl From<InvalidMethod> for Error {
    fn from(_: InvalidMethod) -> Self {
        Self::InvalidRequestLine
    }
}

impl From<InvalidUri> for Error {
    fn from(_: InvalidUri) -> Self {
        Self::InvalidRequestLine
    }
}

impl From<InvalidHeaderName> for Error {
    fn from(_: InvalidHeaderName) -> Self {
        Self::InvalidHeaders
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(_: InvalidHeaderValue) -> Self {
        Self::InvalidHeaders
    }
}
