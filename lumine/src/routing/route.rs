//! Route implementation.
//!
//! This module provides the [`Route`] struct, which is the concrete
//! implementation of a route in Lumine, combining a path, a handler,
//! and route-specific middleware.

use std::sync::Arc;

use crate::{
    middleware::Middleware,
    request::Request,
    request::params::Params,
    response::Response,
    response::into_response::IntoResponse,
    routing::{path::Path, route_service::RouteService},
};

/// A concrete route that matches a path and dispatches to a handler.
pub struct Route<'a, F> {
    pub(crate) path: Path<'a>,
    pub(crate) middlewares: Vec<Arc<dyn Middleware>>,
    pub(crate) run_before_global: bool,
    pub(crate) handler: F,
}

impl<'a, F> Route<'a, F> {
    /// Adds a new middleware to this specific route.
    pub fn middleware<M: Middleware>(mut self, middleware: M) -> Self {
        self.middlewares.push(Arc::new(middleware));
        self
    }

    /// Configures this route to run its own middleware before any global middleware.
    pub fn run_before_global(mut self) -> Self {
        self.run_before_global = true;
        self
    }
}

#[async_trait::async_trait]
impl<'a, F, Fut, R> RouteService for Route<'a, F>
where
    F: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = R> + Send + 'static,
    R: IntoResponse,
{
    fn matches(&self, path: &Path) -> Option<Params> {
        if path.len() != self.path.len() {
            None
        } else {
            let mut params = Params::with_capacity(4);

            for (route_part, path_parts) in self.path.iter().zip(path.as_ref()) {
                if let Some(param_name) = route_part.strip_prefix(':') {
                    params.insert(param_name.to_owned(), (*path_parts).into());
                } else if route_part != path_parts {
                    return None;
                }
            }

            Some(params)
        }
    }
    fn middlewares(&self) -> &[Arc<dyn Middleware>] {
        &self.middlewares
    }
    fn run_before_global(&self) -> bool {
        self.run_before_global
    }
    fn is_duplicated(&self, path: &Path) -> bool {
        *self.path == **path
    }
    async fn call(&self, request: Request) -> Response {
        (self.handler)(request).await.into_response()
    }
}
