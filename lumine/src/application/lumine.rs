//! Main application structure and implementation.
//!
//! This module provides the [`Lumine`] struct, which is the central entry
//! point for building and running a Lumine application. It manages
//! routes, middleware, and server configuration.

use crate::{
    application::{
        limits::Limits,
        states::{Building, Ready},
        timeouts::Timeouts,
    },
    internal::{connection, parser, validator},
    middleware::Middleware,
    request::{Request, params::Params},
    response::into_response::IntoResponse,
    routing::{route::Route, route_entry::RouteEntry},
};
use std::{marker::PhantomData, sync::Arc};
use tokio::net::TcpListener;

/// The main HTTP application structure.
///
/// [`Lumine`] uses a compile-time state system to ensure correct API usage.
/// During the `Building` state, routes and configuration can be added.
/// Once built, the application enters the `Ready` state and can be served.
///
/// This design prevents invalid usage (such as modifying routes after
/// the server has started) at compile time.
pub struct Lumine<State = Building> {
    // Limits
    pub(crate) limits: Limits,

    // Timeouts
    pub(crate) timeouts: Timeouts,

    // Routes and middlewares
    pub(crate) routes: Vec<Arc<dyn RouteEntry>>,
    pub(crate) middlewares: Vec<Arc<dyn Middleware>>,

    // States
    _state: PhantomData<State>,
}

impl Lumine {
    /// Creates a new Lumine application in the configuration phase.
    ///
    /// The returned application is in the [`Building`] state, allowing routes
    /// and server settings to be configured. This is the only state where
    /// modification is permitted.
    pub fn builder() -> Lumine<Building> {
        Lumine {
            limits: Limits::default(),

            timeouts: Timeouts::default(),

            routes: Vec::new(),
            middlewares: Vec::with_capacity(4),

            _state: PhantomData,
        }
    }
}

#[allow(clippy::panic)] // allow panics in the builder
impl Lumine<Building> {
    /// Finalizes the application configuration.
    ///
    /// This method transitions the application from the [`Building`] state
    /// into the [`Ready`] state. After calling this method, routes and
    /// configuration can no longer be modified.
    pub fn build(self) -> Lumine<Ready> {
        Lumine {
            limits: self.limits,

            timeouts: self.timeouts,

            routes: self.routes,
            middlewares: self.middlewares,

            _state: PhantomData,
        }
    }

    /// Specifies the limits for the application.
    pub fn limits(mut self, limits: Limits) -> Self {
        self.limits = limits;
        self
    }

    /// Specifies the timeouts for the application.
    pub fn timeouts(mut self, timeouts: Timeouts) -> Self {
        self.timeouts = timeouts;
        self
    }

    /// Add a new global middleware.
    pub fn middleware<M: Middleware>(mut self, middleware: M) -> Self {
        self.middlewares.push(Arc::new(middleware));
        self
    }

    /// Registers a new route and its handler.
    ///
    /// This method adds a route definition to the application, associating
    /// a path pattern with a handler function. Routes are matched in a
    /// normalized, segment-based manner during request handling.
    ///
    /// # Path requirements
    ///
    /// The provided `path` **must start with a leading slash (`/`)** and
    /// represent a valid route pattern.
    ///
    /// Examples of valid paths:
    ///
    /// ```text
    /// /
    /// /users
    /// /users/:userId
    /// ```
    ///
    /// # Handler
    ///
    /// The `callback` function is invoked when the route matches an incoming
    /// request. It must:
    ///
    /// - Accept a [`Request`] as input
    /// - Return a type that implements [`IntoResponse`]
    /// - Be thread-safe (`Send + Sync`)
    ///
    /// # Panics
    ///
    /// This method will panic in the following cases:
    ///
    /// - If the provided `path` does not start with `/` or is otherwise invalid.
    ///   This is treated as a programmer error and enforced to maintain a
    ///   single canonical form for route definitions.
    ///
    /// - If the route conflicts with an existing route.
    ///   Two routes are considered conflicting if they would match the same
    ///   request paths (e.g. `/users/:id` and `/users/:userId`).
    ///
    /// These panics occur at configuration time rather than runtime to
    /// fail fast and avoid ambiguous routing behavior.
    ///
    /// # Example
    ///
    /// ```rust
    /// use lumine::prelude::*;
    ///
    /// async fn user(req: Request) -> impl IntoResponse {
    ///     "ok"
    /// }
    ///
    /// let app = Lumine::builder()
    ///     .route("/users/:userId", user)
    ///     .build();
    /// ```
    pub fn route<F, Fut, R>(mut self, path: &'static str, handler: F) -> Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = R> + Send,
        R: IntoResponse,
    {
        let segments = parser::parse_path(path, &self.limits);
        validator::check_route_duplicates(&self.routes, &segments);

        self.routes.push(Arc::new(Route {
            segments,
            middlewares: Vec::new(),
            run_before_global: false,
            handler,
        }));

        self
    }

    /// Registers a route with additional per-route configuration.
    ///
    /// This method behaves similarly to [`Lumine::route`], but allows the caller
    /// to modify the constructed route before it is registered.
    pub fn route_with<F, R, Fut, W>(mut self, path: &'static str, handler: F, with: W) -> Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = R> + Send,
        R: IntoResponse,
        W: Fn(Route<F>) -> Route<F>,
    {
        let segments = parser::parse_path(path, &self.limits);
        validator::check_route_duplicates(&self.routes, &segments);

        let route = with(Route {
            segments,
            middlewares: Vec::with_capacity(3),
            run_before_global: false,
            handler,
        });

        self.routes.push(Arc::new(route));
        self
    }
}

impl Lumine<Ready> {
    /// Starts serving incoming HTTP connections.
    ///
    /// This method consumes the application in the [`Ready`] state and begins
    /// accepting connections from the provided [`TcpListener`].
    pub async fn serve(self, listener: TcpListener) {
        let app = Arc::new(self);

        loop {
            if let Ok((stream, _)) = listener.accept().await {
                let app = Arc::clone(&app);
                tokio::spawn(async move { connection::handle_connection(app, stream).await });
            }
        }
    }

    /// Serves incoming TLS connections using the provided [`TcpListener`] and [`ServerConfig`].
    #[cfg(feature = "tls")]
    pub async fn serve_tls(
        self,
        listener: TcpListener,
        config: tokio_rustls::rustls::ServerConfig,
    ) {
        let app = Arc::new(self);
        let acceptor = tokio_rustls::TlsAcceptor::from(Arc::new(config));

        loop {
            if let Ok((stream, _)) = listener.accept().await
                && let Ok(tls_stream) = acceptor.accept(stream).await
            {
                let app = Arc::clone(&app);
                tokio::spawn(async move { connection::handle_connection(app, tls_stream).await });
            }
        }
    }

    /// Returns the route and parameters that matches the given URI, if one exists.
    #[doc(hidden)]
    pub fn get_route(&self, path: &str) -> Option<(Arc<dyn RouteEntry>, Params)> {
        let path = if path == "/" {
            Vec::new()
        } else {
            path.split('/').skip(1).collect::<Vec<&str>>()
        };

        for route in &self.routes {
            if let Some(params) = route.matches(&path) {
                return Some((Arc::clone(route), params));
            }
        }

        None
    }

    /// Returns the application's resource limits.
    pub fn limits(&self) -> Limits {
        self.limits
    }

    /// Returns the application's timeouts.
    pub fn timeouts(&self) -> &Timeouts {
        &self.timeouts
    }
}
