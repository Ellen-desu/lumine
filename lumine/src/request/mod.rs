//! HTTP Request handling for Lumine.
//!
//! This module provides the core [Request] type and utilities for
//! parsing and accessing request components like query parameters
//! and path parameters.

pub mod from_request;
pub mod params;
pub mod query;

use bytes::Bytes;

#[doc(inline)]
pub use self::{from_request::FromRequest, params::Params, query::Query};

/// The request type used throughout Lumine.
///
/// This is an alias for `http::Request<Bytes>`, binding the
/// request body type to [`Bytes`] definition.
///
/// Using a type alias keeps function signatures concise and
/// ensures consistent request handling across the codebase.
pub type Request = http::Request<Bytes>;
