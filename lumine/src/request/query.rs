//! Query parameter extraction.
//!
//! This module provides the [`Query`] struct, which handles the parsing
//! and storage of query parameters from the request URI.

use std::ops::{Deref, DerefMut};

/// Represents query parameters extracted from the request URI.
///
/// `Query` contains key-value pairs parsed from the query string
/// of a request URI (the part after `?`).
///
/// Unlike path parameters, query parameters may appear multiple
/// times with the same key. To preserve this information, each key
/// maps to a `Vec<Box<str>>`.
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
/// # Accessing Query
///
/// Use [`Query::from_request`] to retrieve query parameters.
///
/// # Example
///
/// ```rust
/// use lumine::{Request, IntoResponse, FromRequest, Query};
///
/// fn search(req: Request) -> impl IntoResponse {
///     let query = Query::from_request(&req);
///
///     let search = query.get("search");
///     assert_eq!(search, Some("foobar"));
///
///     "ok"
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Query(Vec<(Box<str>, Vec<Box<str>>)>);

impl Query {
    /// Creates a new `Query`.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Creates a new `Query` with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    /// Inserts a key-value pair into the query.
    pub fn insert(&mut self, key: Box<str>, value: Box<str>) {
        if let Some(values) = self.0.iter_mut().find(|(k, _)| *k == key) {
            values.1.push(value);
        } else {
            self.0.push((key, vec![value]));
        }
    }

    /// Retrieves the first value for the given key, if present.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0
            .iter()
            .find(|(k, _)| k.as_ref() == key)
            .map(|(_, v)| v[0].as_ref())
    }

    /// Retrieves all values for the given key, if present.
    pub fn get_all(&self, key: &str) -> Option<impl Iterator<Item = &str>> {
        self.0
            .iter()
            .find(|(k, _)| k.as_ref() == key)
            .map(|(_, v)| v.iter().map(|v| v.as_ref()))
    }
}

impl Deref for Query {
    type Target = Vec<(Box<str>, Vec<Box<str>>)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Query {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
