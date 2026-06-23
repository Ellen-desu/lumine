//! HTTP Response handling for Lumine.
//!
//! This module defines the [Response] type used throughout the
//! application and provides the [IntoResponse] trait for converting
//! handler results into valid HTTP responses.

pub mod into_response;

pub(crate) mod default_headers;

#[doc(inline)]
pub use self::into_response::IntoResponse;

/// The response type produced by handlers and the server runtime.
///
/// This is an alias for `http::Response<DynBody>`, representing a
/// fully constructed HTTP response.
///
/// All values returned from handlers are ultimately converted
/// into this type via the `IntoResponse` trait.
pub type Response = http::Response<crate::body::DynBody>;
