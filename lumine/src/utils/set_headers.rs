//! Header manipulation utilities.
//!
//! This module provides the [`SetHeaders`] trait, which offers a convenient
//! way to apply multiple headers from a [`HeaderMap`] to
//! request or response builders.

use http::{HeaderMap, request, response};

/// A trait for types that can set headers on a request or response builder.
pub trait SetHeaders {
    /// Applies the headers from the given [`HeaderMap`] to the builder.
    fn headers(self, headers: &HeaderMap) -> Self;
}

impl SetHeaders for response::Builder {
    fn headers(mut self, headers: &HeaderMap) -> Self {
        for (key, value) in headers.iter() {
            self = self.header(key, value);
        }

        self
    }
}

impl SetHeaders for request::Builder {
    fn headers(mut self, headers: &HeaderMap) -> Self {
        for (key, value) in headers.iter() {
            self = self.header(key, value);
        }

        self
    }
}
