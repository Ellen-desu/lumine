//! HTTP response body representation.
//!
//! This module defines the [`Body`] enum, which represents the various forms
//! an HTTP response body can take in Lumine. It supports:
//!
//! - [`Body::Empty`]: No content.
//! - [`Body::Bytes`]: Buffered data (e.g., `Vec<u8>`).
//! - [`Body::Stream`]: Data provided by a type implementing the [`Stream`] trait.

pub mod into_body;

#[doc(inline)]
pub use self::into_body::IntoBody;

use crate::stream::Stream;

/// Represents the body of an HTTP response.
pub enum Body<S: Stream> {
    /// An empty response body.
    Empty,
    /// A response body containing buffered bytes.
    Bytes(Vec<u8>),
    /// A response body that streams data using the [`Stream`] trait.
    Stream(S),
}

impl<S: Stream> Body<S> {
    /// Returns `true` if the body is empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty) || matches!(self, Self::Bytes(bytes) if bytes.is_empty())
    }

    /// Returns a reference to the body as bytes, if it is a `Bytes` variant.
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Self::Bytes(bytes) => Some(bytes),
            _ => None,
        }
    }

    /// Returns the body as bytes, if it is a `Bytes` variant.
    pub fn into_bytes(self) -> Option<Vec<u8>> {
        match self {
            Self::Bytes(bytes) => Some(bytes),
            _ => None,
        }
    }

    /// Returns a `Stream` variant of the body.
    pub fn from_stream(stream: S) -> Self {
        Self::Stream(stream)
    }

    /// Returns a `Bytes` variant of the body.
    pub fn from_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self::Bytes(bytes.into())
    }

    /// Returns an empty variant of the body.
    pub fn empty() -> Self {
        Self::Empty
    }
}

/// A type-erased response body.
///
/// `DynBody` is a [`Body`] that holds a boxed [`Stream`] trait object.
/// This allows handlers to return different types of streams in a single
/// response type.
pub type DynBody = Body<Box<dyn Stream>>;
