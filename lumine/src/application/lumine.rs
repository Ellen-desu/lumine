use crate::{
    application::{
        client::Client,
        states::{Builder, Ready},
    },
    internal::handler,
    routing::{params::Params, path::Path, query::Query, route::Route},
    traits::{into_response::IntoResponse, route_service::RouteService},
    types::request::Request,
};
use http::Uri;
use std::{
    marker::PhantomData,
    net::TcpListener,
    sync::{
        Arc,
        mpsc::{self, Receiver},
    },
    thread,
    time::Duration,
};

type R = Box<dyn RouteService + Send + Sync>;

/// The main HTTP application structure.
///
/// `Lumine` uses a compile-time state system to ensure correct API usage.
/// During the `Builder` state, routes and configuration can be added.
/// Once built, the application enters the `Ready` state and can be served.
///
/// This design prevents invalid usage (such as modifying routes after
/// the server has started) at compile time.
pub struct Lumine<State = Builder> {
    routes: Vec<R>,
    timeout: Option<Duration>,
    max_body: usize,
    _state: PhantomData<State>,
}

impl Lumine {
    /// Creates a new Lumine application in the configuration phase.
    ///
    /// The returned application is in the `Builder` state, allowing routes
    /// and server settings to be configured. This is the only state where
    /// modification is permitted.
    pub fn builder() -> Lumine<Builder> {
        Lumine {
            routes: Vec::new(),
            timeout: None,
            max_body: 1024, // 1KB
            _state: PhantomData::<Builder>,
        }
    }
}

impl Lumine<Builder> {
    /// Finalizes the application configuration.
    ///
    /// This method transitions the application from the `Builder` state
    /// into the `Ready` state. After calling this method, routes and
    /// configuration can no longer be modified.
    pub fn build(self) -> Lumine<Ready> {
        Lumine {
            routes: self.routes,
            timeout: self.timeout,
            max_body: self.max_body,
            _state: PhantomData::<Ready>,
        }
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
        let path = Path::from(path);
        for route in &self.routes {
            if route.is_duplicated(&path) {
                panic!("Conflicting routes");
            }
        }

        self.routes.push(Box::new(Route { path, handler }));
        self
    }

    /// Specifies maximum body size in bytes.
    pub fn max_body_size(mut self, max: usize) -> Self {
        self.max_body = max;
        self
    }
    /// Set read and write `TcpStream` timeout.
    ///
    /// # Panics
    ///
    /// Panics if the zero [`Duration`] is passed to this method.
    pub fn set_timeout(mut self, duration: Duration) -> Self {
        if duration.is_zero() {
            panic!("The timeout duration can't be zero.");
        }
        self.timeout = Some(duration);
        self
    }
}

impl Lumine<Ready> {
    /// Starts serving incoming HTTP connections.
    ///
    /// This method consumes the application in the `Ready` state and begins
    /// accepting connections from the provided `TcpListener`.
    ///
    /// The returned [`Receiver`] **must** be continuously polled to keep the
    /// internal event loop alive. Dropping or ignoring it will cause the
    /// server to stop processing events.
    pub fn serve(self, listener: TcpListener) -> Receiver<Client> {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let app = Arc::new(self);
            let tx = Arc::new(tx);

            for stream_result in listener.incoming() {
                let app = Arc::clone(&app);
                let tx = Arc::clone(&tx);

                if let Ok(stream) = stream_result {
                    let _ = stream.set_read_timeout(app.timeout);
                    let _ = stream.set_write_timeout(app.timeout);

                    thread::spawn(move || {
                        let _ = handler::handle_client(app, stream, tx);
                    });
                }
            }
        });

        // Returning the receiver for error handling
        rx
    }

    pub(crate) fn get_route(&self, uri: &Uri) -> Option<(&R, Params, Query)> {
        let mut query = Query::default();

        if let Some(raw_query) = uri.query() {
            for (key, value) in form_urlencoded::parse(raw_query.as_bytes()).into_owned() {
                query
                    .entry(key.to_string())
                    .or_default()
                    .push(value.to_string());
            }
        }

        let path_parts = Path::from(uri.path());
        for route in &self.routes {
            if let Some(params) = route.matches(&path_parts) {
                return Some((route, params, query));
            }
        }

        None
    }

    /// Same as `Lumine::get_route` method but only for performance testing.
    #[cfg(feature = "bench")]
    pub fn get_route_for_bench(&self, uri: &Uri) -> Option<(&R, Params, Query)> {
        self.get_route(uri)
    }

    /// Get the application maximum body that have been set
    pub fn max_body(&self) -> usize {
        self.max_body
    }
}
