//! Route service abstraction.
//!
//! This module defines the [`RouteService`] trait, which abstracts the
//! behavior of a single route, including matching, middleware execution,
//! and handler dispatching.

use crate::{
    middleware::Middleware,
    routing::{params::Params, path::Path},
    types::{request::Request, response::Response, result::Result},
};

/// Defines the behavior of a single route within the routing system.
///
/// RouteService abstracts how a route:
///
/// - Determines whether it matches a request path
/// - Detects conflicts with other routes
/// - Provides route-specific middleware
/// - Dispatches a matched request to its handler
///
/// This trait allows the routing system to operate on routes
/// without knowing their concrete implementation.
///
/// # Role in routing
///
/// During request handling, the router interacts with routes
/// through this trait in the following order:
///
/// 1. [RouteService::matches] is called to determine whether the route matches
///    the incoming request path and to extract path parameters.
/// 2. If a match is found, the router constructs a middleware chain consisting of:
/// - Application-level middleware
/// - Route-specific middleware (from [RouteService::middlewares])
/// 3. The execution order of the middleware chain is determined by
///    [RouteService::route_middleware_first].
/// 4. The request is passed into the middleware chain via a Next executor.
/// 5. After all middleware have been executed, [RouteService::call] is invoked
///    as the final step.
///
/// During route registration, [RouteService::is_duplicated] is used to prevent
/// ambiguous or conflicting route definitions.
///
/// # Middleware execution model
///
/// Middleware are executed in a chained manner, where each middleware
/// receives control along with a Next instance representing the remaining chain.
///
/// A middleware may:
///
/// - Forward the request to the next middleware
/// - Short-circuit the chain by returning a response early
///
/// This design enables flexible behaviors such as authentication,
/// logging, and request transformation.
///
/// # Design
///
/// RouteService intentionally separates:
///
/// - Matching logic (path comparison and parameter extraction)
/// - Middleware configuration (route-specific middleware and ordering)
/// - Dispatch logic (invoking the handler)
///
/// This separation keeps routing predictable and makes route implementations
/// easier to reason about, extend, and refactor.
pub trait RouteService: Send + Sync {
    /// Attempts to match the given path segments against this route.
    ///
    /// Returns `Some(Params)` if the path matches, containing any
    /// extracted path parameters. Returns `None` if the route does
    /// not match.
    ///
    /// This method performs pure matching logic and does not
    /// invoke the route handler.
    fn matches(&self, path: &Path) -> Option<Params>;

    /// Determines whether this route conflicts with another route
    /// using the given path pattern.
    ///
    /// This is primarily used during route registration to prevent
    /// ambiguous routes that would match the same request paths.
    fn is_duplicated(&self, path: &Path) -> bool;

    /// Returns the middleware configured specifically for this route.
    ///
    /// These middleware will be combined with application-level middleware
    /// during request execution.
    fn middlewares(&self) -> &[Box<dyn Middleware>];

    /// Indicates whether route-specific middleware should be executed
    /// before application-level middleware.
    ///
    /// If `true`, route middleware are executed first.
    /// If `false`, application middleware are executed first.
    fn route_middleware_first(&self) -> bool;

    /// Invokes the route handler with the provided request.
    ///
    /// This method is called only after the route has successfully
    /// matched the request path. It is responsible for executing
    /// the handler and converting its return value into an HTTP
    /// response.
    fn call(&self, request: Request) -> Result<Response>;
}
