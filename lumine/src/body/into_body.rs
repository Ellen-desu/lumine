//! Body conversion trait.
//!
//! This module provides the [`IntoBody`] trait, which allows various types
//! to be converted into a [`DynBody`]. This is used to make returning
//! different types of content from handlers more ergonomic.

use crate::body::{Body, DynBody};
use std::str::Bytes;

/// Converts a value into an HTTP-compatible body.
///
/// This trait serves as a boundary between high-level application types
/// (such as strings) and the lower-level HTTP layer, which operates
/// purely on bytes.
///
/// # Design
///
/// HTTP bodies are byte-oriented by nature. Rather than forcing handlers
/// to manually convert values into [`Body`], [`IntoBody`] provides a small
/// abstraction that allows common types to be returned naturally.
///
/// This keeps handler code ergonomic while preserving a clear separation
/// between application logic and HTTP encoding.
pub trait IntoBody {
    /// Converts the value into a vector of raw bytes.
    fn into_body(self) -> DynBody;
}

impl IntoBody for &'static str {
    fn into_body(self) -> DynBody {
        Body::Bytes(self.as_bytes().to_vec())
    }
}

impl IntoBody for &[u8] {
    fn into_body(self) -> DynBody {
        Body::Bytes(self.to_vec())
    }
}

impl IntoBody for String {
    fn into_body(self) -> DynBody {
        Body::Bytes(self.into_bytes())
    }
}

impl IntoBody for Vec<u8> {
    fn into_body(self) -> DynBody {
        Body::Bytes(self)
    }
}

impl IntoBody for &String {
    fn into_body(self) -> DynBody {
        Body::Bytes(self.as_bytes().to_vec())
    }
}

impl IntoBody for Bytes<'_> {
    fn into_body(self) -> DynBody {
        Body::Bytes(self.collect())
    }
}

impl IntoBody for DynBody {
    fn into_body(self) -> DynBody {
        self
    }
}

#[cfg(feature = "filestream")]
impl IntoBody for crate::filestream::FileStream {
    fn into_body(self) -> DynBody {
        Body::Stream(Box::new(self))
    }
}
