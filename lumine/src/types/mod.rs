//! Core type aliases used throughout Lumine.
//!
//! This module defines a set of type aliases that standardize commonly
//! used types across the codebase.
//!
//! The goal of these aliases is to:
//!
//! - Reduce repetition in function signatures
//! - Centralize important type decisions
//! - Provide a lightweight abstraction boundary for future changes
//!
//! These aliases do not introduce new behavior. They exist purely to
//! improve readability, consistency, and maintainability.
//!
//! # Overview
//!
//! The most important aliases defined in this module are:
//!
//! - [`Body`]: The concrete type used for HTTP message bodies.
//! - [`Request`]: The request type used by handlers and routing.
//! - [`Response`]: The response type produced by handlers and the server.
//! - [`Result`]: The standard result type used across Lumine.
//!
//! By using these aliases consistently, Lumine can evolve internal
//! representations (such as the body type or error model) without
//! requiring widespread changes to public APIs.
//!
//! # Design notes
//!
//! - HTTP is fundamentally byte-oriented, which is reflected in the
//!   definition of [`Body`].
//! - Requests and responses are aliases over `http` crate types to
//!   avoid reimplementing low-level HTTP semantics.
//! - The [`Result`] alias ensures a single error type is used
//!   throughout the framework.

pub mod body;
pub mod request;
pub mod response;
pub mod result;

pub use body::Body;
pub use request::Request;
pub use response::Response;
pub use result::Result;
