pub mod next;

pub use self::next::Next;

use crate::types::{request::Request, response::Response, result::Result};

pub trait Middleware: Send + Sync + 'static {
    /// Handles an incoming HTTP request within the middleware chain.
    ///
    /// This method is called for each middleware in sequence. It receives
    /// the current [`Request`] and a [`Next`] handler that represents the
    /// remaining middleware stack (including the final route handler).
    fn handle(&self, request: Request, next: Next) -> Result<Response>;
}
