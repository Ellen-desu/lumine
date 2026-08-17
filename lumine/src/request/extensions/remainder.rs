//! Wildcard path remainder extraction.
//!
//! This module provides the [`Remainder`] struct, which holds the trailing
//! segments of the request path when a route ends with a wildcard (`*`).
//!
//! If a route has a wildcard, everything after the matching static/param
//! segments is collected into a `Remainder`. This is particularly useful
//! for serving static files or handling arbitrary sub-paths.

/// Represents the trailing path matched by a wildcard route segment.
///
/// `Remainder` contains the rest of the request path that was caught by a
/// wildcard (`*`) in the route definition.
///
/// Like other request extensions, it is automatically extracted during
/// routing and can be accessed inside a handler using the
/// [`FromRequest`](crate::request::FromRequest) trait.
///
/// # Example
///
/// If you have a route `/files/*`:
/// - A request to `/files/css/style.css` will yield a `Remainder` of `css/style.css`.
/// - A request to `/files/` will yield an empty `Remainder`.
///
/// ```rust
/// use lumine::{FromRequest, IntoResponse, Remainder, Request};
///
/// async fn serve_file(req: Request) -> impl IntoResponse {
///     let remainder = Remainder::from_request(&req);
///     let path = remainder.get().unwrap_or("index.html");
///     format!("Serving: {}", path)
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct Remainder(Option<Box<str>>);

impl Remainder {
    /// Creates a new, empty `Remainder`.
    pub fn new() -> Self {
        Self(None)
    }

    /// Returns the matched remainder path as a string slice.
    ///
    /// Returns `None` if the wildcard matched an empty path, or if the
    /// route did not contain a wildcard at all.
    pub fn get(&self) -> Option<&str> {
        self.0.as_deref()
    }

    /// Returns `true` if the remainder is empty or not present.
    pub fn is_empty(&self) -> bool {
        self.0.as_deref().is_none_or(str::is_empty)
    }
}

impl From<Box<str>> for Remainder {
    fn from(value: Box<str>) -> Self {
        Self(Some(value))
    }
}
