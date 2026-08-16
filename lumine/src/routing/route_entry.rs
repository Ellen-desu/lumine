//! Route service abstraction.
//!
//! This module defines the [`RouteEntry`] trait, which abstracts the
//! behavior of a single route, including matching, middleware execution,
//! and handler dispatching.

use std::sync::Arc;

use crate::{
    middleware::Middleware,
    request::{
        Request,
        extensions::{Remainder, params::Params},
    },
    response::{Response, into_response::IntoResponse},
    routing::{route::Route, segment::Segment},
};

/// Defines the behavior of a single route within the routing system.
///
/// RouteEntry abstracts how a route:
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
/// 1. [RouteEntry::matches] is called to determine whether the route matches
///    the incoming request path and to extract path parameters.
/// 2. If a match is found, the router constructs a middleware chain consisting of:
/// - Application-level middleware
/// - Route-specific middleware (from [RouteEntry::middlewares])
/// 3. The execution order of the middleware chain is determined by
///    [RouteEntry::run_before_global].
/// 4. The request is passed into the middleware chain via a Next executor.
/// 5. After all middleware have been executed, [RouteEntry::call] is invoked
///    as the final step.
///
/// During route registration, [RouteEntry::is_duplicated] is used to prevent
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
/// RouteEntry intentionally separates:
///
/// - Matching logic (path comparison and parameter extraction)
/// - Middleware configuration (route-specific middleware and ordering)
/// - Dispatch logic (invoking the handler)
///
/// This separation keeps routing predictable and makes route implementations
/// easier to reason about, extend, and refactor.
#[async_trait::async_trait]
pub trait RouteEntry: Send + Sync {
    /// Attempts to match the given path segments against this route.
    ///
    /// Returns `Some(Params)` if the path matches, containing any
    /// extracted path parameters. Returns `None` if the route does
    /// not match.
    ///
    /// This method performs pure matching logic and does not
    /// invoke the route handler.
    fn matches(&self, path_parts: &[&str]) -> Option<(Params, Remainder)>;

    /// Determines whether this route conflicts with another route
    /// using the given path pattern.
    ///
    /// This is primarily used during route registration to prevent
    /// ambiguous routes that would match the same request paths.
    fn is_duplicated(&self, segments: &[Segment]) -> bool;

    /// Returns the middleware configured specifically for this route.
    ///
    /// These middleware will be combined with global-level middleware
    /// during request execution.
    fn middlewares(&self) -> &[Arc<dyn Middleware>];

    /// Indicates whether route-specific middleware should be executed
    /// before global-level middleware.
    ///
    /// If `true`, route middlewares are executed first.
    /// If `false`, global middlewares are executed first.
    fn run_before_global(&self) -> bool;

    /// Invokes the route handler with the provided request.
    ///
    /// This method is called only after the route has successfully
    /// matched the request path. It is responsible for executing
    /// the handler and converting its return value into an HTTP
    /// response.
    async fn call(&self, request: Request) -> Response;
}

#[async_trait::async_trait]
impl<F, Fut, R> RouteEntry for Route<F>
where
    F: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = R> + Send,
    R: IntoResponse,
{
    fn matches(&self, path_parts: &[&str]) -> Option<(Params, Remainder)> {
        let wildcard = self.segments.ends_with(&[Segment::Wildcard]);
        if (!wildcard && path_parts.len() != self.segments.len())
            || (wildcard && path_parts.len() < self.segments.len())
        {
            return None;
        }

        let mut params = Params::with_capacity(4);
        let mut remainder = Remainder::new();

        for (i, (segment, part)) in self.segments.iter().zip(path_parts).enumerate() {
            match segment {
                Segment::Static(s) => {
                    if s != part {
                        return None;
                    }
                }
                Segment::Param(param_name) => {
                    params.insert(param_name, part.to_string().into_boxed_str());
                }
                Segment::Wildcard => {
                    remainder = Remainder::from(path_parts[i..].join("/").into_boxed_str());
                    break;
                }
            }
        }

        Some((
            if params.is_empty() {
                Params::new()
            } else {
                params
            },
            remainder,
        ))
    }
    fn middlewares(&self) -> &[Arc<dyn Middleware>] {
        &self.middlewares
    }
    fn run_before_global(&self) -> bool {
        self.run_before_global
    }
    fn is_duplicated(&self, segments: &[Segment]) -> bool {
        self.segments.len() == segments.len()
            && self
                .segments
                .iter()
                .zip(segments)
                .all(|(a, b)| match (a, b) {
                    (Segment::Static(x), Segment::Static(y)) => x == y,
                    (Segment::Param(_), Segment::Param(_)) => true,
                    _ => a == &Segment::Wildcard || b == &Segment::Wildcard,
                })
    }
    async fn call(&self, request: Request) -> Response {
        (self.handler)(request).await.into_response()
    }
}
