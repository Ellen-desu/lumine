//! HTTP framing and connection state module.
//!
//! This module defines structures that represent the framing and connection
//! lifecycle of an HTTP request.

/// Represents the framing information of an HTTP request.
///
/// This struct holds parsed information about how the request body is framed,
/// such as the content length, and the desired connection state based on headers.
#[derive(Debug, Clone, PartialEq)]
pub struct Framing {
    /// The length of the request body in bytes, if specified in the headers.
    pub content_length: Option<usize>,
    /// The desired connection state (e.g., Keep-Alive or Close).
    pub connection: Connection,
}

/// Represents the state of an HTTP connection.
///
/// This enum indicates whether a connection should be kept open for further
/// requests or closed after the current response is sent.
#[derive(Debug, Clone, PartialEq)]
pub enum Connection {
    /// The connection should be kept alive for subsequent requests.
    KeepAlive,
    /// The connection should be closed after the current response.
    Close,
}

impl Connection {
    /// Returns `true` if the connection state is `KeepAlive`.
    pub fn is_keep_alive(&self) -> bool {
        matches!(self, Connection::KeepAlive)
    }

    /// Returns `true` if the connection state is `Close`.
    pub fn is_close(&self) -> bool {
        matches!(self, Connection::Close)
    }
}
