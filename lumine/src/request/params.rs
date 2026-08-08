//! Path parameter extraction.
//!
//! This module provides the [`Params`] struct, which holds dynamic segments
//! extracted from the request path during routing.

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
    /// Creates a new `Params`.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Creates a new `Params` with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    /// Inserts a key-value pair into the `Params`.
    pub fn insert(&mut self, key: &'static str, value: Box<str>) {
        self.0.push((key, value));
    }

    /// Returns the value associated with the given key, if one exists.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v.as_ref())
    }

    /// Returns whether the `Params` contains a value for the given key.
    pub fn contains_key(&self, key: &str) -> bool {
        self.0.iter().any(|(k, _)| *k == key)
    }

    /// Returns an iterator over the keys in the `Params`.
    pub fn keys(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.0.iter().map(|(k, _)| *k)
    }

    /// Returns an iterator over the values in the `Params`.
    pub fn values(&self) -> impl Iterator<Item = &str> + '_ {
        self.0.iter().map(|(_, v)| v.as_ref())
    }

    /// Returns the number of key-value pairs in the `Params`.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns whether the `Params` is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
