//! Path parameter extraction.
//!
//! This module provides the [`Params`] struct, which holds dynamic segments
//! extracted from the request path during routing.

use crate::types::request::Request;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

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
/// # String-only values
///
/// All path parameter values are stored as `String`.
/// Routing operates purely on textual data, and converting values
/// into concrete types (e.g. `u32`) is intentionally left to
/// application logic.
///
/// # Accessing Params
///
/// Use [`Params::from_request`] to retrieve parameters from a request.
/// This returns `None` if the matched route does not define any
/// dynamic path segments.
///
/// # Example
///
/// ```rust
/// use lumine::{Request, IntoResponse, Params};
///
/// fn user(req: Request) -> impl IntoResponse {
///     let params = Params::from_request(&req)
///         .expect("route must define path parameters");
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
pub struct Params(HashMap<String, String>);

impl Params {
    /// Retrieves path parameters from the request extensions.
    ///
    /// Returns `None` if no path parameters were attached to the request,
    /// which usually means the matched route does not contain dynamic
    /// segments.
    pub fn from_request(request: &Request) -> Option<&Self> {
        request.extensions().get::<Self>()
    }
}

impl DerefMut for Params {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Params {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
