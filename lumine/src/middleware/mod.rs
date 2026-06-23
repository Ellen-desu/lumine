//! Middleware system for request and response processing.
//!
//! Middleware provides a way to wrap the execution of route handlers with
//! additional logic. This can be used for tasks like logging, authentication,
//! CORS, and more.
//!
//! The core of the middleware system is the [`Middleware`] trait.

pub mod next;

#[doc(inline)]
pub use self::next::Next;

use crate::{request::Request, response::Response};

/// Trait for creating HTTP middleware.
///
/// Middleware acts as an intermediate layer between an incoming request
/// and the application's final route handler.
///
/// Each middleware receives:
///
/// - [`Request`] → the current HTTP request
/// - [`Next`] → the remaining middleware chain
#[async_trait::async_trait]
pub trait Middleware: Send + Sync + 'static {
    /// Handles an incoming HTTP request within the middleware chain.
    ///
    /// This method is called for each middleware in sequence. It receives
    /// the current [`Request`] and a [`Next`] handler that represents the
    /// remaining middleware stack (including the final route handler).
    async fn handle(&self, request: Request, next: Next) -> Response;
}
