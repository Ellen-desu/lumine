//! Query parameter extraction.
//!
//! This module provides the [`Query`] struct, which stores the key/value pairs
//! parsed from the query string of an incoming request URI (the portion after
//! `?`).
//!
//! Query parameters are populated by Lumine automatically and can be retrieved
//! inside a handler via the [`FromRequest`](crate::request::FromRequest) trait.

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
/// 5. The handler function is invoked with the populated [`Request`](crate::request::Request).
///
/// As a result, [`Query`] is only available inside handlers.
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
    /// Creates a new, empty `Query` collection.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Creates a new, empty `Query` collection with space pre-allocated for
    /// at least `capacity` unique keys, avoiding reallocations during parsing.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    /// Inserts a query parameter key-value pair.
    ///
    /// If `key` already exists, `value` is appended to its value list,
    /// preserving all occurrences (e.g. `?tag=rust&tag=web` results in
    /// `tag → ["rust", "web"]`). Otherwise a new entry is created.
    pub fn insert(&mut self, key: Box<str>, value: Box<str>) {
        if let Some(values) = self.0.iter_mut().find(|(k, _)| *k == key) {
            values.1.push(value);
        } else {
            self.0.push((key, vec![value]));
        }
    }

    /// Returns the **first** value associated with `key`, or `None` if the key
    /// is not present.
    ///
    /// When the same key appears multiple times in the query string, use
    /// [`get_all`](Self::get_all) to retrieve all values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use lumine::{FromRequest, IntoResponse, Query, Request};
    ///
    /// async fn search(req: Request) -> impl IntoResponse {
    ///     let query = Query::from_request(&req);
    ///     let term = query.get("q").unwrap_or("(empty)");
    ///     format!("Searching for: {term}")
    /// }
    /// ```
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0
            .iter()
            .find(|(k, _)| k.as_ref() == key)
            .map(|(_, v)| v[0].as_ref())
    }

    /// Returns an iterator over **all** values associated with `key`, or `None`
    /// if the key is not present.
    ///
    /// Useful when the same query key is repeated, such as
    /// `?tag=rust&tag=web`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use lumine::{FromRequest, IntoResponse, Query, Request};
    ///
    /// async fn tags(req: Request) -> impl IntoResponse {
    ///     let query = Query::from_request(&req);
    ///     let tags: Vec<&str> = query
    ///         .get_all("tag")
    ///         .map(|it| it.collect())
    ///         .unwrap_or_default();
    ///     format!("Tags: {}", tags.join(", "))
    /// }
    /// ```
    pub fn get_all(&self, key: &str) -> Option<impl Iterator<Item = &str>> {
        self.0
            .iter()
            .find(|(k, _)| k.as_ref() == key)
            .map(|(_, v)| v.iter().map(|v| v.as_ref()))
    }

    /// Returns `true` if the query string contains at least one value for `key`.
    pub fn contains_key(&self, key: &str) -> bool {
        self.0.iter().any(|(k, _)| k.as_ref() == key)
    }

    /// Returns an iterator over every unique key in the query string, in the
    /// order they first appeared.
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(|(k, _)| k.as_ref())
    }

    /// Returns an iterator over every value list in the query string, in the
    /// same order as [`keys`](Self::keys). Each item is a slice of all values
    /// recorded for that key.
    pub fn values(&self) -> impl Iterator<Item = &[Box<str>]> {
        self.0.iter().map(|(_, v)| v.as_slice())
    }

    /// Returns an iterator over every `(key, values)` pair in the query string,
    /// in the order the keys first appeared.
    pub fn iter(&self) -> impl Iterator<Item = &(Box<str>, Vec<Box<str>>)> {
        self.0.iter()
    }

    /// Returns the number of **unique** keys in the query string.
    ///
    /// Note that a key repeated multiple times (e.g. `?tag=rust&tag=web`)
    /// counts as one.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the query string contained no parameters.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'a> IntoIterator for &'a Query {
    type Item = &'a (Box<str>, Vec<Box<str>>);
    type IntoIter = std::slice::Iter<'a, (Box<str>, Vec<Box<str>>)>;

    /// Iterates over every `(key, values)` pair in insertion order.
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

