//! Resource limits and constraints.
//!
//! This module defines the [`Limits`] struct, which specifies various
//! constraints for the HTTP server, such as maximum path size, header
//! counts, and body size. These limits help protect the server against
//! resource exhaustion and certain types of DoS attacks.

/// Resource limits for the HTTP server.
#[derive(Debug, Clone, Copy)]
pub struct Limits {
    /// Maximum size of the request path.
    pub max_path_size: usize,
    /// Maximum total size of query parameters in bytes.
    pub max_query_size: usize,
    /// Maximum number of query parameters allowed.
    pub max_query_count: usize,
    /// Maximum total size of all request headers in bytes.
    pub max_headers_size: usize,
    /// Maximum number of request headers allowed.
    pub max_headers_count: usize,
    /// Maximum size of a path segment.
    pub max_segment_size: usize,
    /// Maximum number of path segments allowed.
    pub max_segments_count: usize,
    /// Maximum size of the request body in bytes.
    pub max_body_size: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_path_size: 2048, // 2 KB

            max_query_size: 8 * 1024, // 8 KB
            max_query_count: 100,

            max_headers_size: 32 * 1024, // 32 KB
            max_headers_count: 100,

            max_segment_size: 256, // 256 bytes
            max_segments_count: 100,

            max_body_size: 10 * 1024 * 1024, // 10 MB
        }
    }
}
