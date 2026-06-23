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
    pub request_read: Duration,
    /// Timeout for writing the response to the client.
    pub response_write: Duration,
    /// Timeout for reading data from a stream.
    pub stream_read: Duration,
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
