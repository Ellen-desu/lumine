//! Path parameter extraction.
//!
//! This module provides the [`Params`] struct, which holds dynamic segments
//! extracted from the request path during routing.

use std::ops::{Deref, DerefMut};

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
/// 5. The handler function is invoked with the populated [`Request`].
///
/// As a result, [`Params`] is only available inside handlers and
/// only when the matched route defines path parameters.
///
/// # Accessing Params
///
/// Use [`Params::from_request`] to retrieve parameters from a request.
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

    /// Returns the value associated with the given key, if one exists.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v.as_ref())
    }
}

impl DerefMut for Params {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Params {
    type Target = Vec<(&'static str, Box<str>)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
