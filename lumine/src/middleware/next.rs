//! Middleware execution chain.
//!
//! This module provides the [`Next`] struct, which represents the remaining
//! middleware pipeline and the final route handler. It allows middleware to
//! control the flow of execution by deciding when (or if) to call the next
//! step in the chain.

use crate::{
    middleware::Middleware,
    routing::route_service::RouteService,
    types::{request::Request, response::Response, result::Result},
};

/// Represents the remaining execution chain of middlewares.
///
/// [`Next`] is responsible for advancing the request through the middleware
/// pipeline. It holds a slice of remaining middlewares and the final
/// [`RouteService`] handler.
///
/// This struct is typically passed into [`Middleware::handle`] and is used
/// to delegate execution to the next middleware or the final route handler.
pub struct Next<'a> {
    pub(crate) middlewares: &'a [&'a dyn Middleware],
    pub(crate) route: &'a dyn RouteService,
}

impl<'a> Next<'a> {
    #[doc(hidden)]
    pub fn new(middlewares: &'a [&'a dyn Middleware], route: &'a dyn RouteService) -> Self {
        Self { middlewares, route }
    }

    /// Executes the next step in the middleware chain.
    pub fn run(self, request: Request) -> Result<Response> {
        if let Some((first, rest)) = self.middlewares.split_first() {
            let next = Next {
                middlewares: rest,
                route: self.route,
            };

            first.handle(request, next)
        } else {
            self.route.call(request)
        }
    }
}
