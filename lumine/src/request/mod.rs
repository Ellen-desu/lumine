//! HTTP Request handling for Lumine.
//!
//! This module provides the core [Request] type and utilities for
//! parsing and accessing request components like query parameters
//! and path parameters.

pub mod extensions;
pub mod from_request;

use bytes::Bytes;

#[doc(inline)]
pub use self::{
    extensions::{addr::Addr, params::Params, query::Query},
    from_request::FromRequest,
};

/// The request type used throughout Lumine.
///
/// This is an alias for `http::Request<Bytes>`, binding the
/// request body type to [`Bytes`] definition.
///
/// Using a type alias keeps function signatures concise and
/// ensures consistent request handling across the codebase.
pub type Request = http::Request<Bytes>;
