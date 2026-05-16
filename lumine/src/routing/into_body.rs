use crate::types::body::Body;
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
/// to manually convert values into [`Body`], `IntoBody` provides a small
/// abstraction that allows common types to be returned naturally.
///
/// This keeps handler code ergonomic while preserving a clear separation
/// between application logic and HTTP encoding.
///
/// # Implementations
///
/// Lumine provides built-in implementations for common string types:
///
/// - `&'static str`
/// - `String`
/// - '&String'
/// - 'Bytes'
///
/// These are encoded as UTF-8 byte sequences.
///
/// Additional implementations (e.g. borrowed strings, byte buffers,
/// or custom types) can be added without changing handler signatures.
///
/// # Example
///
/// ```rust
/// use lumine::routing::into_body::IntoBody;
///
/// let body = "Hello, world!".into_body();
///
/// assert_eq!(body, b"Hello, world!");
/// ```
///
/// # Notes
///
/// - `IntoBody` does not perform any content-type detection or encoding
///   beyond producing raw bytes.
/// - Higher-level concerns such as headers and status codes are handled
///   elsewhere.
pub trait IntoBody {
    /// Converts the value into a vector of raw bytes.
    fn into_body(self) -> Body;
}

impl IntoBody for &'static str {
    fn into_body(self) -> Body {
        self.as_bytes().to_vec()
    }
}

impl IntoBody for String {
    fn into_body(self) -> Body {
        self.into_bytes()
    }
}

impl IntoBody for &String {
    fn into_body(self) -> Body {
        self.as_bytes().into()
    }
}

impl IntoBody for Bytes<'_> {
    fn into_body(self) -> Body {
        self.collect()
    }
}
