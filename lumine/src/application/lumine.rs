use crate::{
    application::{
        limits::Limits,
        states::{Building, Ready},
    },
    internal::handler,
    middleware::Middleware,
    routing::{
        into_response::IntoResponse, params::Params, path::Path, route::Route,
        route_service::RouteService,
    },
    types::{request::Request, result::Result},
};
use http::Uri;
use std::{marker::PhantomData, net::TcpListener, sync::Arc, thread, time::Duration};

/// The main HTTP application structure.
///
/// `Lumine` uses a compile-time state system to ensure correct API usage.
/// During the `Building` state, routes and configuration can be added.
/// Once built, the application enters the `Ready` state and can be served.
///
/// This design prevents invalid usage (such as modifying routes after
/// the server has started) at compile time.
pub struct Lumine<State = Building> {
    // Limits
    pub(crate) limits: Limits,

    // Routes and middlewares
    pub(crate) routes: Vec<Box<dyn RouteService>>,
    pub(crate) middlewares: Vec<Box<dyn Middleware>>,

    // Timeouts
    pub(crate) read_timeout: Duration,
    pub(crate) write_timeout: Duration,

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

            routes: Vec::new(),
            middlewares: Vec::new(),

            read_timeout: Duration::from_secs(10),
            write_timeout: Duration::from_secs(10),

            _state: PhantomData,
        }
    }
}

impl Lumine<Building> {
    /// Finalizes the application configuration.
    ///
    /// This method transitions the application from the [`Building`] state
    /// into the [`Ready`] state. After calling this method, routes and
    /// configuration can no longer be modified.
    pub fn build(self) -> Lumine<Ready> {
        Lumine {
            limits: self.limits,

            routes: self.routes,
            middlewares: self.middlewares,

            read_timeout: self.read_timeout,
            write_timeout: self.write_timeout,

            _state: PhantomData,
        }
    }

    /// Specifies the limits for the application.
    pub fn limits(mut self, limits: Limits) -> Self {
        self.limits = limits;
        self
    }

    /// Specifies maximum uri size in bytes.
    pub fn max_uri_size(mut self, max: usize) -> Self {
        self.limits.max_uri_size = max;
        self
    }

    /// Specifies maximum headers size in bytes.
    pub fn max_headers_size(mut self, max: usize) -> Self {
        self.limits.max_headers_size = max;
        self
    }

    /// Specifies maximum headers count.
    pub fn max_headers_count(mut self, max: usize) -> Self {
        self.limits.max_headers_count = max;
        self
    }

    /// Specifies maximum query size in bytes.
    pub fn max_query_size(mut self, max: usize) -> Self {
        self.limits.max_query_size = max;
        self
    }

    /// Specifies maximum query count.
    pub fn max_query_count(mut self, max: usize) -> Self {
        self.limits.max_query_count = max;
        self
    }

    /// Specifies maximum body size in bytes.
    pub fn max_body_size(mut self, max: usize) -> Self {
        self.limits.max_body_size = max;
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
    /// use lumine::{Lumine, Request, IntoResponse};
    ///
    /// fn user(req: Request) -> impl IntoResponse {
    ///     "ok"
    /// }
    ///
    /// let app = Lumine::builder()
    ///     .route("/users/:userId", user)
    ///     .build();
    /// ```
    pub fn route<F, R>(mut self, path: &'static str, handler: F) -> Self
    where
        F: Fn(Request) -> R + Send + Sync + 'static,
        R: IntoResponse,
    {
        if path.len() > self.limits.max_uri_size {
            panic!("URI too long");
        }

        let path = Path::from(path);

        if self.routes.iter().any(|r| r.is_duplicated(&path)) {
            panic!("Conflicting routes");
        }

        self.routes.push(Box::new(Route {
            path,
            middlewares: Vec::new(),
            route_middleware_first: false,
            handler,
        }));

        self
    }

    /// Registers a route with additional per-route configuration.
    ///
    /// This method behaves similarly to [`Lumine::route`], but allows the caller
    /// to modify the constructed route before it is registered.
    pub fn route_with<F, R, W>(mut self, path: &'static str, handler: F, with: W) -> Self
    where
        F: Fn(Request) -> R + Send + Sync + 'static,
        R: IntoResponse,
        W: Fn(Route<F>) -> Route<F> + Send + Sync + 'static,
    {
        let path = Path::from(path);
        for route in &self.routes {
            if route.is_duplicated(&path) {
                panic!("Conflicting routes");
            }
        }

        let route = with(Route {
            path,
            middlewares: Vec::new(),
            route_middleware_first: false,
            handler,
        });

        self.routes.push(Box::new(route));
        self
    }

    /// Add a new global middleware.
    pub fn middleware<M: Middleware>(mut self, middleware: M) -> Self {
        self.middlewares.push(Box::new(middleware));
        self
    }

    /// Set read `TcpStream` timeout.
    ///
    /// # Panics
    ///
    /// Panics if the zero [`Duration`] is passed to this method.
    pub fn read_timeout(mut self, duration: Duration) -> Self {
        if duration.is_zero() {
            panic!("The timeout duration can't be zero.");
        }
        self.read_timeout = duration;
        self
    }

    /// Set write `TcpStream` timeout.
    ///
    /// # Panics
    ///
    /// Panics if the zero [`Duration`] is passed to this method.
    pub fn write_timeout(mut self, duration: Duration) -> Self {
        if duration.is_zero() {
            panic!("The timeout duration can't be zero.");
        }
        self.write_timeout = duration;
        self
    }
}

impl Lumine<Ready> {
    /// Starts serving incoming HTTP connections.
    ///
    /// This method consumes the application in the `Ready` state and begins
    /// accepting connections from the provided `TcpListener`.
    pub fn serve(self, listener: TcpListener) -> Result<()> {
        let app = Arc::new(self);

        for stream_result in listener.incoming() {
            let app = Arc::clone(&app);

            if let Ok(stream) = stream_result {
                stream.set_read_timeout(Some(app.read_timeout))?;
                stream.set_write_timeout(Some(app.write_timeout))?;

                thread::spawn(move || {
                    let _ = handler::handle_client(app, stream);
                });
            }
        }

        Ok(())
    }

    pub(crate) fn get_route(&self, uri: &Uri) -> Option<(&dyn RouteService, Params)> {
        let path_parts = Path::from(uri.path());
        for route in &self.routes {
            if let Some(params) = route.matches(&path_parts) {
                return Some((route.as_ref(), params));
            }
        }

        None
    }

    /// Get the application limits
    pub fn limits(&self) -> Limits {
        self.limits
    }

    /// Get the application maximum request line that have been set
    pub fn max_uri_size(&self) -> usize {
        self.limits.max_uri_size
    }

    /// Get the application maximum query size that have been set
    pub fn max_query_size(&self) -> usize {
        self.limits.max_query_size
    }

    /// Get the application maximum headers size that have been set
    pub fn max_headers_size(&self) -> usize {
        self.limits.max_headers_size
    }

    /// Get the application maximum headers count that have been set
    pub fn max_headers_count(&self) -> usize {
        self.limits.max_headers_count
    }

    /// Get the application maximum body that have been set
    pub fn max_body_size(&self) -> usize {
        self.limits.max_body_size
    }

    #[cfg(feature = "bench")]
    pub fn get_route_for_bench(&self, uri: &Uri) -> Option<(&dyn RouteService, Params)> {
        self.get_route(uri)
    }
}
