//! Routing primitives and path matching utilities.
//!
//! This module defines the core building blocks of Lumine's routing system.
//! Routing is responsible for matching incoming request URIs against
//! registered routes, extracting parameters, and dispatching the request
//! to the appropriate handler.
//!
//! # Overview
//!
//! At a high level, routing in Lumine works as follows:
//!
//! 1. An incoming HTTP request is received.
//! 2. The request URI is normalized and segmented into a [`Path`].
//! 3. The router attempts to match the request path against registered routes.
//! 4. If a route matches:
//!    - Path parameters are extracted into [`Params`].
//!    - Query parameters are parsed into [`Query`].
//! 5. The matched route's handler is invoked with the populated [`Request`].
//!
//! This module focuses purely on **path matching and parameter extraction**.
//! Higher-level concerns such as request parsing, response writing, and
//! error handling are handled elsewhere.
//!
//! # Core Types
//!
//! ## [`Path`]
//!
//! A normalized, segmented representation of a URL path.
//!
//! - Splits a raw path string into individual segments.
//! - Enforces a leading slash (`/`) invariant.
//! - Normalizes trailing slashes to avoid ambiguous routes.
//!
//! `Path` is used internally during route registration and request matching
//! to ensure consistent and predictable behavior.
//!
//! ## [`Params`]
//!
//! A collection of path parameters extracted from dynamic route segments.
//!
//! For example, given a route definition:
//!
//! ```text
//! /users/:userId
//! ```
//!
//! And an incoming request:
//!
//! ```text
//! GET /users/42
//! ```
//!
//! The routing system will extract:
//!
//! ```text
//! userId = "42"
//! ```
//!
//! All parameter values are stored as `String`. Type conversion is
//! intentionally left to application logic.
//!
//! ## [`Query`]
//!
//! Represents query parameters parsed from the request URI.
//!
//! Unlike path parameters, query parameters may appear multiple times
//! with the same key. To preserve this information, each key maps to a
//! `Vec<String>`.
//!
//! Example:
//!
//! ```text
//! /search?tag=rust&tag=web
//! ```
//!
//! Results in:
//!
//! ```text
//! tag = ["rust", "web"]
//! ```
//!
//! # Design Principles
//!
//! - **Single canonical path representation**
//!   All paths must start with `/` and are normalized before matching.
//!
//! - **Separation of concerns**
//!   Routing only decides *which* handler to call and *what parameters*
//!   to extract. It does not execute handlers directly.
//!
//! - **Minimal assumptions**
//!   Parameters are treated as raw strings. Interpretation and validation
//!   are handled at the application layer.
//!
//! This strict separation keeps the routing core small, predictable,
//! and easy to refactor.

pub mod params;
pub mod path;
pub mod query;
pub mod route;

pub mod into_body;
pub mod into_response;
pub mod route_service;

pub use self::{
    into_response::IntoResponse, params::Params, path::Path, query::Query, route::Route,
};

#[allow(unused)]
use crate::types::request::Request;
