//! HTTP Request handling for Lumine.
//!
//! This module provides the core [Request] type and utilities for
//! parsing and accessing request components like query parameters
//! and path parameters.

pub mod params;
pub mod query;

#[doc(inline)]
pub use self::{params::Params, query::Query};

/// The request type used throughout Lumine.
///
/// This is an alias for `http::Request<Vec<u8>>`, binding the
/// request body type to [`Vec<u8>`] definition.
///
/// Using a type alias keeps function signatures concise and
/// ensures consistent request handling across the codebase.
pub type Request = http::Request<Vec<u8>>;
