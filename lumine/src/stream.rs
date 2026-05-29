//! Core trait for data streaming.
//!
//! This module provides the [`Stream`] trait, which is the primary abstraction
//! for handling streaming data within Lumine. It allows for reading data in
//! chunks, which is essential for handling large response bodies or file transfers
//! without loading the entire content into memory.

use crate::types::result::Result;

/// A trait for types that can provide data in chunks.
///
/// This is used by [`Body::Stream`](crate::body::Body::Stream) to send
/// data asynchronously or in pieces, avoiding the need to load the entire
/// content into memory at once.
pub trait Stream {
    /// Attempts to read the next chunk of data into the provided buffer.
    ///
    /// Returns the number of bytes read on success, or an error if the
    /// operation fails. A return value of `0` indicates the end of the stream.
    fn next_chunk(&mut self, buffer: &mut [u8]) -> Result<usize>;

    /// Returns a hint about the total size of the stream in bytes, if known.
    ///
    /// This value is used to set the `Content-Length` header in the response.
    /// If `None` is returned, the response will be sent using `Transfer-Encoding: chunked`.
    fn size_hint(&self) -> Option<usize> {
        None
    }
}

impl<T: Stream + ?Sized> Stream for Box<T> {
    fn next_chunk(&mut self, buf: &mut [u8]) -> Result<usize> {
        (**self).next_chunk(buf)
    }
}
