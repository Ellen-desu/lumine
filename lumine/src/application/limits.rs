//! Resource limits and constraints.
//!
//! This module defines the [`Limits`] struct, which specifies various
//! constraints for the HTTP server, such as maximum URI size, header
//! counts, and body size. These limits help protect the server against
//! resource exhaustion and certain types of DoS attacks.

/// Resource limits for the HTTP server.
#[derive(Debug, Clone, Copy)]
pub struct Limits {
    /// Maximum size of the request URI in bytes.
    pub max_uri_size: usize,
    /// Maximum total size of query parameters in bytes.
    pub max_query_size: usize,
    /// Maximum number of query parameters allowed.
    pub max_query_count: usize,
    /// Maximum total size of all request headers in bytes.
    pub max_headers_size: usize,
    /// Maximum number of request headers allowed.
    pub max_headers_count: usize,
    /// Maximum size of the request body in bytes.
    pub max_body_size: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_uri_size: 8 * 1024, // 8 KB

            // Maximum query size is 800 KB
            max_query_size: 8 * 1024, // 8 KB
            max_query_count: 100,

            // Maximum headers size is 6.4 MB
            max_headers_size: 64 * 1024, // 64 KB
            max_headers_count: 100,

            max_body_size: 10 * 1024 * 1024, // 10 MB
        }
    }
}
