//! Middleware execution chain.
//!
//! This module provides the [`Next`] struct, which represents the remaining
//! middleware pipeline and the final route handler. It allows middleware to
//! control the flow of execution by deciding when (or if) to call the next
//! step in the chain.

use crate::{
    middleware::Middleware, request::Request, response::Response, routing::route_entry::RouteEntry,
};
use std::sync::Arc;

/// Represents the remaining execution chain of middlewares.
///
/// [`Next`] is responsible for advancing the request through the middleware
/// pipeline. It holds a slice of remaining middlewares and the final
/// [`RouteEntry`] handler.
///
/// This struct is typically passed into [`Middleware::handle`] and is used
/// to delegate execution to the next middleware or the final route handler.
pub struct Next {
    pub(crate) middlewares: Arc<[Arc<dyn Middleware>]>,
    pub(crate) route: Arc<dyn RouteEntry>,
    pub(crate) index: usize,
}

impl Next {
    #[doc(hidden)]
    pub fn new(middlewares: Vec<Arc<dyn Middleware>>, route: Arc<dyn RouteEntry>) -> Self {
        Self {
            middlewares: Arc::<[Arc<dyn Middleware>]>::from(middlewares),
            route,
            index: 0,
        }
    }

    /// Executes the next step in the middleware chain.
    pub async fn run(self, request: Request) -> Response {
        if let Some(middleware) = self.middlewares.get(self.index) {
            let next = Next {
                middlewares: Arc::clone(&self.middlewares),
                route: self.route,
                index: self.index + 1,
            };

            middleware.handle(request, next).await
        } else {
            self.route.call(request).await
        }
    }
}
