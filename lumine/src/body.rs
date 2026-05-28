use crate::stream::Stream;

pub enum Body<S: Stream> {
    Empty,
    Bytes(Vec<u8>),
    Stream(S),
}

impl<S: Stream> Body<S> {
    /// Returns `true` if the body is empty, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
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
    pub fn stream(stream: S) -> Self {
        Self::Stream(stream)
    }

    /// Returns a `Bytes` variant of the body.
    pub fn bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self::Bytes(bytes.into())
    }

    /// Returns an empty variant of the body.
    pub fn empty() -> Self {
        Self::Empty
    }
}
