pub mod next;

pub use self::next::Next;

use crate::types::{request::Request, response::Response, result::Result};

/// Trait for creating HTTP middleware.
///
/// Middleware acts as an intermediate layer between an incoming request
/// and the application's final route handler.
///
/// Each middleware receives:
///
/// - [`Request`] → the current HTTP request
/// - [`Next`] → the remaining middleware chain
pub trait Middleware: Send + Sync + 'static {
    /// Handles an incoming HTTP request within the middleware chain.
    ///
    /// This method is called for each middleware in sequence. It receives
    /// the current [`Request`] and a [`Next`] handler that represents the
    /// remaining middleware stack (including the final route handler).
    fn handle(&self, request: Request, next: Next) -> Result<Response>;
}
