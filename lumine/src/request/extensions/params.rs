//! Path parameter extraction.
//!
//! This module provides the [`Params`] struct, which holds the dynamic path
//! segments extracted from the request URI during route matching.
//!
//! Params are populated by Lumine automatically and can be retrieved inside a
//! handler via the [`FromRequest`](crate::request::FromRequest) trait.

/// Represents path parameters extracted from a matched route.
///
/// `Params` contains key-value pairs parsed from dynamic segments
/// in a route definition, such as `:userId` in `/:userId`.
///
/// This struct does **not** parse raw URLs by itself.
/// Path parameters are extracted during the routing phase while
/// matching the request URI against registered routes, and are
/// automatically inserted into the request's extensions **before**
/// the handler is called.
///
/// # Lifecycle
///
/// For an incoming request handled by Lumine:
///
/// 1. The HTTP request is parsed.
/// 2. The request URI is matched against registered routes.
/// 3. If the matched route defines dynamic segments (e.g. `:userId`),
///    their values are extracted.
/// 4. A [`Params`] instance is created and attached to the request.
/// 5. The handler function is invoked with the populated [`Request`](crate::request::Request).
///
/// As a result, [`Params`] is only available inside handlers and
/// only when the matched route defines path parameters.
///
/// # Example
///
/// ```rust
/// use lumine::{Request, IntoResponse, FromRequest, Params};
///
/// fn user(req: Request) -> impl IntoResponse {
///     let params = Params::from_request(&req);
///
///     let user_id = params
///         .get("userId")
///         .expect("userId param is required");
///
///     let user_id: u32 = user_id.parse().unwrap();
///
///     format!("User id: {}", user_id)
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Params(Vec<(&'static str, Box<str>)>);

impl Params {
    /// Creates a new, empty `Params` collection.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Creates a new, empty `Params` collection with space pre-allocated for
    /// at least `capacity` entries, avoiding reallocations during population.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    /// Inserts a path parameter key-value pair.
    ///
    /// `key` must be a `'static` string because route segment names are
    /// defined at compile time (e.g. `":userId"`). `value` is the
    /// corresponding segment captured from the incoming request path.
    ///
    /// Duplicate keys are **not** deduplicated; [`get`](Self::get) returns
    /// the first match.
    pub fn insert(&mut self, key: &'static str, value: Box<str>) {
        self.0.push((key, value));
    }

    /// Returns the value of the first parameter matching `key`, or `None` if
    /// no such parameter exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use lumine::{FromRequest, IntoResponse, Params, Request};
    ///
    /// async fn user(req: Request) -> impl IntoResponse {
    ///     let params = Params::from_request(&req);
    ///     let id = params.get("userId").expect("userId param is required");
    ///     format!("User: {id}")
    /// }
    /// ```
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v.as_ref())
    }

    /// Returns `true` if at least one parameter with the given `key` exists.
    pub fn contains_key(&self, key: &str) -> bool {
        self.0.iter().any(|(k, _)| *k == key)
    }

    /// Returns an iterator over every parameter name in insertion order.
    pub fn keys(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.0.iter().map(|(k, _)| *k)
    }

    /// Returns an iterator over every `(key, value)` pair in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = &(&'static str, Box<str>)> {
        self.0.iter()
    }

    /// Returns an iterator over every parameter value in insertion order.
    pub fn values(&self) -> impl Iterator<Item = &str> + '_ {
        self.0.iter().map(|(_, v)| v.as_ref())
    }

    /// Returns the total number of path parameters.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if no path parameters have been captured.
    ///
    /// This is the case for routes with no dynamic segments (e.g. `/health`).
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'a> IntoIterator for &'a Params {
    type Item = &'a (&'static str, Box<str>);
    type IntoIter = std::slice::Iter<'a, (&'static str, Box<str>)>;

    /// Iterates over every `(key, value)` pair in insertion order.
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
