//! Routing primitives and path matching utilities.
//!
//! This module defines the core building blocks of Lumine's routing system.
//! Routing is responsible for matching incoming request URIs against
//! registered routes, and dispatching the request to the appropriate handler.

#![doc(hidden)]

pub mod route;
pub mod route_entry;
pub mod segment;

pub use self::{route::Route, route_entry::RouteEntry};
