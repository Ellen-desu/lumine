//! Path normalization and segmentation.
//!
//! This module provides the [`Path`] struct, which is used to represent
//! and manipulate normalized HTTP request paths.

use std::ops::{Deref, DerefMut};

/// Represents a normalized, segmented view of a URL path.
///
/// `Path` splits a raw path string into individual segments that
/// can be used for routing and pattern matching.
///
/// This type performs **path normalization** and enforces a strict
/// invariant to ensure consistent route definitions.
///
/// # Invariant
///
/// All paths **must start with a leading slash (`/`)**.
///
/// Providing a path that does not start with `/` will cause a
/// panic at construction time. This is intentional and treated
/// as a programmer error.
///
/// Enforcing this invariant avoids ambiguous route definitions
/// and eliminates the need to handle multiple equivalent path
/// representations internally.
///
/// # Normalization rules
///
/// Given a valid path string:
///
/// - The leading `/` is ignored for segmentation.
/// - Empty segments are removed.
/// - A trailing `/` is normalized away.
///
/// This means the following paths are treated equivalently:
///
/// ```text
/// /users
/// /users/
/// ```
///
/// Both will result in the same internal representation.
///
/// # Example
///
/// ```rust
/// use lumine::Path;
///
/// let path = Path::from("/users/:id");
///
/// assert_ne!(&path, &Path::from("/"));
/// assert_eq!(*path, vec!["users", ":id"]);
/// ```
///
/// # Panics
///
/// Panics if the provided path does not start with `/`.
///
/// # Design notes
///
/// - The first empty segment produced by splitting on `/` is
///   intentionally discarded, as it carries no semantic meaning.
/// - Path segments are stored as `&str` slices instead of owned
///   `String`s to avoid unnecessary allocations during routing.
/// - This type is primarily intended for internal use by the
///   routing system.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Path<'a>(Vec<&'a str>);

impl<'a> DerefMut for Path<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> Deref for Path<'a> {
    type Target = Vec<&'a str>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> AsRef<[&'a str]> for Path<'a> {
    fn as_ref(&self) -> &[&'a str] {
        &self.0
    }
}

#[allow(clippy::panic)]
impl<'a> From<&'a str> for Path<'a> {
    fn from(value: &'a str) -> Self {
        if !value.starts_with("/") {
            panic!("Path can't be empty or doesn't start with \"/\"")
        }

        Self({
            let mut parts = value.split("/").collect::<Vec<&str>>();

            // The first index is always empty or "". So, i remove it :)
            parts.remove(0);

            // Normalize trailing slashes:
            // "/users/" -> ["users"]
            let len = parts.len();
            if let Some(&"") = parts.last()
                && len > 1
            {
                parts.remove(len - 1);
            }

            parts
        })
    }
}
