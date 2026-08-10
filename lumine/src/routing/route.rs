//! Route implementation.
//!
//! This module provides the [`Route`] struct, which is the concrete
//! implementation of a route in Lumine, combining a path, a handler,
//! and route-specific middleware.

use std::sync::Arc;

use crate::{middleware::Middleware, routing::segment::Segment};

/// A concrete route that matches a path and dispatches to a handler.
pub struct Route<F> {
    pub(crate) segments: Vec<Segment>,
    pub(crate) middlewares: Vec<Arc<dyn Middleware>>,
    pub(crate) run_before_global: bool,
    pub(crate) handler: F,
}

impl<F> Route<F> {
    /// Registers a new route with the given path and handler.
    #[doc(hidden)]
    pub fn new(segments: Vec<Segment>, middlewares: Vec<Arc<dyn Middleware>>, handler: F) -> Self {
        Self {
            segments,
            middlewares,
            run_before_global: false,
            handler,
        }
    }

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
