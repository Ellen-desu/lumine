//! Body conversion trait.
//!
//! This module provides the [`IntoBody`] trait, which allows various types
//! to be converted into a [`DynBody`]. This is used to make returning
//! different types of content from handlers more ergonomic.

use bytes::{Bytes, BytesMut};

use crate::{
    body::{Body, DynBody},
    stream::Stream,
};

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
        Body::Bytes(Bytes::from_static(self.as_bytes()))
    }
}

impl IntoBody for &'static [u8] {
    fn into_body(self) -> DynBody {
        Body::Bytes(Bytes::from_static(self))
    }
}

impl IntoBody for Box<[u8]> {
    fn into_body(self) -> DynBody {
        Body::Bytes(Bytes::from(self))
    }
}

impl IntoBody for Bytes {
    fn into_body(self) -> DynBody {
        Body::Bytes(self)
    }
}

impl IntoBody for BytesMut {
    fn into_body(self) -> DynBody {
        Body::Bytes(self.freeze())
    }
}

impl IntoBody for Vec<u8> {
    fn into_body(self) -> DynBody {
        Body::Bytes(Bytes::from(self))
    }
}

impl IntoBody for String {
    fn into_body(self) -> DynBody {
        Body::Bytes(Bytes::from(self))
    }
}

impl IntoBody for DynBody {
    /// Returns the body directly.
    fn into_body(self) -> DynBody {
        self
    }
}

impl<S: Stream + 'static> IntoBody for S {
    fn into_body(self) -> DynBody {
        Body::Stream(Box::new(self))
    }
}
