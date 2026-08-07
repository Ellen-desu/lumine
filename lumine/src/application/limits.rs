//! Resource limits and constraints.
//!
//! This module defines the [`Limits`] struct, which specifies various
//! constraints for the HTTP server, such as maximum path size, header
//! counts, and body size. These limits help protect the server against
//! resource exhaustion and certain types of DoS attacks.

/// Resource limits for the HTTP server.
#[derive(Debug, Clone, Copy)]
pub struct Limits {
    /// Maximum number of connections allowed.
    pub(crate) max_connections: usize,
    /// Maximum size of the request path.
    pub(crate) max_path_size: usize,
    /// Maximum total size of query parameters in bytes.
    pub(crate) max_query_size: usize,
    /// Maximum number of query parameters allowed.
    pub(crate) max_query_count: usize,
    /// Maximum total size of all request headers in bytes.
    pub(crate) max_headers_size: usize,
    /// Maximum number of request headers allowed.
    pub(crate) max_headers_count: usize,
    /// Maximum size of a path segment.
    pub(crate) max_segment_size: usize,
    /// Maximum number of path segments allowed.
    pub(crate) max_segments_count: usize,
    /// Maximum size of the request body in bytes.
    pub(crate) max_body_size: usize,
}

impl Limits {
    /// Specifies the maximum number of connections for the application.
    ///
    /// # Panics
    ///
    /// Panics if `max_connections` is zero.
    pub fn max_connections(mut self, max_connections: usize) -> Self {
        assert!(
            max_connections > 0,
            "max_connections must be greater than 0"
        );
        self.max_connections = max_connections;
        self
    }

    /// Sets the maximum size of the request path.
    ///
    /// # Panics
    ///
    /// Panics if `max_path_size` is zero.
    pub fn max_path_size(mut self, max_path_size: usize) -> Self {
        assert!(max_path_size > 0, "max_path_size must be greater than 0");

        self.max_path_size = max_path_size;
        self
    }

    /// Sets the maximum total size of query parameters in bytes.
    ///
    /// # Panics
    ///
    /// Panics if `max_query_size` is zero.
    pub fn max_query_size(mut self, max_query_size: usize) -> Self {
        assert!(max_query_size > 0, "max_query_size must be greater than 0");
        self.max_query_size = max_query_size;
        self
    }

    /// Sets the maximum number of query parameters allowed.
    ///
    /// # Panics
    ///
    /// Panics if `max_query_count` is zero.
    pub fn max_query_count(mut self, max_query_count: usize) -> Self {
        assert!(
            max_query_count > 0,
            "max_query_count must be greater than 0"
        );
        self.max_query_count = max_query_count;
        self
    }

    /// Sets the maximum total size of all request headers in bytes.
    ///
    /// # Panics
    ///
    /// Panics if `max_headers_size` is zero.
    pub fn max_headers_size(mut self, max_headers_size: usize) -> Self {
        assert!(
            max_headers_size > 0,
            "max_headers_size must be greater than 0"
        );
        self.max_headers_size = max_headers_size;
        self
    }

    /// Sets the maximum number of request headers allowed.
    ///
    /// # Panics
    ///
    /// Panics if `max_headers_count` is zero.
    pub fn max_headers_count(mut self, max_headers_count: usize) -> Self {
        assert!(
            max_headers_count > 0,
            "max_headers_count must be greater than 0"
        );
        self.max_headers_count = max_headers_count;
        self
    }

    /// Sets the maximum size of a path segment.
    ///
    /// # Panics
    ///
    /// Panics if `max_segment_size` is zero.
    pub fn max_segment_size(mut self, max_segment_size: usize) -> Self {
        assert!(
            max_segment_size > 0,
            "max_segment_size must be greater than 0"
        );
        self.max_segment_size = max_segment_size;
        self
    }

    /// Sets the maximum number of path segments allowed.
    ///
    /// # Panics
    ///
    /// Panics if `max_segments_count` is zero.
    pub fn max_segments_count(mut self, max_segments_count: usize) -> Self {
        assert!(
            max_segments_count > 0,
            "max_segments_count must be greater than 0"
        );
        self.max_segments_count = max_segments_count;
        self
    }

    /// Sets the maximum size of the request body in bytes.
    ///
    /// # Panics
    ///
    /// Panics if `max_body_size` is zero.
    pub fn max_body_size(mut self, max_body_size: usize) -> Self {
        assert!(max_body_size > 0, "max_body_size must be greater than 0");
        self.max_body_size = max_body_size;
        self
    }
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_connections: 10_000,

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
