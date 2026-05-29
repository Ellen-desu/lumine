//! Route implementation.
//!
//! This module provides the [`Route`] struct, which is the concrete
//! implementation of a route in Lumine, combining a path, a handler,
//! and route-specific middleware.

use crate::{
    middleware::Middleware,
    routing::{
        into_response::IntoResponse, params::Params, path::Path, route_service::RouteService,
    },
    types::{request::Request, response::Response, result::Result},
};

/// A concrete route that matches a path and dispatches to a handler.
pub struct Route<'a, F> {
    pub(crate) path: Path<'a>,
    pub(crate) middlewares: Vec<Box<dyn Middleware>>,
    pub(crate) route_middleware_first: bool,
    pub(crate) handler: F,
}

impl<'a, F> Route<'a, F> {
    /// Adds a new middleware to this specific route.
    pub fn middleware<M: Middleware>(mut self, middleware: M) -> Self {
        self.middlewares.push(Box::new(middleware));
        self
    }

    /// Configures this route to run its own middleware before any global middleware.
    pub fn route_middleware_first(mut self) -> Self {
        self.route_middleware_first = true;
        self
    }
}

impl<'a, F, R> RouteService for Route<'a, F>
where
    F: Fn(Request) -> R + Send + Sync + 'static,
    R: IntoResponse,
{
    fn matches(&self, path: &Path) -> Option<Params> {
        if path.len() != self.path.len() {
            None
        } else {
            let mut params = Params::default();

            for (route_part, path_parts) in self.path.iter().zip(path.as_ref()) {
                if route_part.starts_with(":") {
                    // If the route path is starts with ":", then take it as parameter
                    params.insert(
                        route_part.strip_prefix(":").unwrap().to_owned(),
                        (*path_parts).into(),
                    );
                } else if route_part != path_parts {
                    return None;
                }
            }

            Some(params)
        }
    }
    fn middlewares(&self) -> &[Box<dyn Middleware>] {
        // self.middlewares.iter().map(|b| b.as_ref()).collect()
        &self.middlewares
    }
    fn route_middleware_first(&self) -> bool {
        self.route_middleware_first
    }
    fn is_duplicated(&self, path: &Path) -> bool {
        *self.path == **path
    }
    fn call(&self, request: Request) -> Result<Response> {
        (self.handler)(request).into_response()
    }
}
