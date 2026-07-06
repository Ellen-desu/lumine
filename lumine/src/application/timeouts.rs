//! Timeout configuration for the Lumine application.
//!
//! This module provides the [`Timeouts`] struct, which defines the duration limits
//! for various I/O operations within the application.

use tokio::time::Duration;

/// Configuration for operation timeouts.
///
/// This struct holds the duration limits for reading requests, writing responses,
/// and reading from streams.
#[derive(Debug, Clone)]
pub struct Timeouts {
    /// Timeout for reading the request from the client.
    pub(crate) request_read: Duration,
    /// Timeout for writing the response to the client.
    pub(crate) response_write: Duration,
    /// Timeout for reading data from a stream.
    pub(crate) stream_read: Duration,
}

impl Timeouts {
    /// Sets the timeout for reading the request from the client.
    ///
    /// # Panics
    ///
    /// Panics if `request_read` is zero.
    pub fn request_read(mut self, request_read: Duration) -> Self {
        assert!(
            !request_read.is_zero(),
            "request_read timeout must be greater than 0"
        );
        self.request_read = request_read;
        self
    }

    /// Sets the timeout for writing the response to the client.
    ///
    /// # Panics
    ///
    /// Panics if `response_write` is zero.
    pub fn response_write(mut self, response_write: Duration) -> Self {
        assert!(
            !response_write.is_zero(),
            "response_write timeout must be greater than 0"
        );
        self.response_write = response_write;
        self
    }

    /// Sets the timeout for reading data from a stream.
    ///
    /// # Panics
    ///
    /// Panics if `stream_read` is zero.
    pub fn stream_read(mut self, stream_read: Duration) -> Self {
        assert!(
            !stream_read.is_zero(),
            "stream_read timeout must be greater than 0"
        );
        self.stream_read = stream_read;
        self
    }
}

impl Default for Timeouts {
    /// Creates a new `Timeouts` instance with default values (30 seconds for all operations).
    fn default() -> Self {
        Timeouts {
            request_read: Duration::from_secs(30),
            response_write: Duration::from_secs(30),
            stream_read: Duration::from_secs(30),
        }
    }
}
