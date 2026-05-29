//! Query parameter extraction.
//!
//! This module provides the [`Query`] struct, which handles the parsing
//! and storage of query parameters from the request URI.

use crate::types::request::Request;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

/// Represents query parameters extracted from the request URI.
///
/// `Query` contains key-value pairs parsed from the query string
/// of a request URI (the part after `?`).
///
/// Unlike path parameters, query parameters may appear multiple
/// times with the same key. To preserve this information, each key
/// maps to a `Vec<String>`.
///
/// # Example
///
/// Given the following request URI:
///
/// ```text
/// /search?tag=rust&tag=web&sort=asc
/// ```
///
/// The extracted query parameters will be:
///
/// ```text
/// tag  = ["rust", "web"]
/// sort = ["asc"]
/// ```
///
/// This struct does **not** parse raw query strings by itself.
/// Query parameters are extracted during the routing phase while
/// matching the request URI, and are automatically inserted into
/// the request's extensions **before** the handler is called.
///
/// # Lifecycle
///
/// For each incoming request:
///
/// 1. The HTTP request is parsed.
/// 2. The request URI is matched against registered routes.
/// 3. The query string is parsed into a [`Query`] structure.
/// 4. The [`Query`] instance is attached to the request.
/// 5. The handler function is invoked with the populated [`Request`].
///
/// As a result, [`Query`] is only available inside handlers.
///
/// # String-only values
///
/// All query parameter values are stored as `String`.
/// Converting values into concrete types (e.g. `bool`, `u32`)
/// is the responsibility of application logic.
///
/// # Accessing Query
///
/// Use [`Query::from_request`] to retrieve query parameters
/// from a request. This returns `None` if the request URI
/// does not contain a query string.
///
/// # Example
///
/// ```rust
/// use lumine::{Request, IntoResponse, Query};
///
/// fn search(req: Request) -> impl IntoResponse {
///     let query = Query::from_request(&req)
///         .expect("request must contain query parameters");
///
///     if let Some(tags) = query.get("tag") {
///         for tag in tags {
///             println!("tag = {}", tag);
///         }
///     }
///
///     "ok"
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Query(HashMap<String, Vec<String>>);

impl Query {
    /// Retrieves query parameters from the request extensions.
    ///
    /// Returns `None` if no query parameters were attached to the
    /// request, which usually means the request URI does not
    /// contain a query string.
    pub fn from_request(request: &Request) -> Option<&Self> {
        request.extensions().get::<Self>()
    }
}

impl DerefMut for Query {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Query {
    type Target = HashMap<String, Vec<String>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
