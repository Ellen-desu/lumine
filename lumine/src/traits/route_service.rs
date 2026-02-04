use crate::{
    routing::{params::Params, path::Path},
    types::{request::Request, response::Response, result::Result},
};

/// Defines the behavior of a single route within the routing system.
///
/// `RouteService` abstracts how a route:
///
/// - Determines whether it matches a request path
/// - Detects conflicts with other routes
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
/// 1. [`RouteService::matches`] is called to determine whether the route matches
///    the incoming request path and to extract path parameters.
/// 2. If a match is found, the request is prepared and passed to
///    [`RouteService::call`] to invoke the route handler.
///
/// During route registration, [`RouteService::is_duplicated`] is used to prevent
/// ambiguous or conflicting route definitions.
///
/// # Design
///
/// `RouteService` intentionally separates:
///
/// - **Matching logic** (path comparison and parameter extraction)
/// - **Dispatch logic** (invoking the handler)
///
/// This keeps routing predictable and makes route implementations
/// easier to reason about and refactor.
pub trait RouteService {
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

    /// Invokes the route handler with the provided request.
    ///
    /// This method is called only after the route has successfully
    /// matched the request path. It is responsible for executing
    /// the handler and converting its return value into an HTTP
    /// response.
    fn call(&self, request: Request) -> Result<Response>;
}
