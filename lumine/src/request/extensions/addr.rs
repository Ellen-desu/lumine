//! Client address extension.
//!
//! This module provides the [`Addr`] struct, which wraps the remote IP address
//! of the client that sent an HTTP request. Lumine attaches an `Addr` to every
//! incoming request's extension map so that handlers and middleware can inspect
//! or log the origin of a connection without depending on lower-level socket
//! primitives.

use std::net::IpAddr;

/// The remote IP address of the client that initiated the request.
///
/// `Addr` is a thin, copy-able wrapper around [`IpAddr`]. Lumine inserts one
/// into the request's extension map before dispatching the request to any
/// middleware or handler, so it is always available inside a handler.
///
/// # Retrieving the address
///
/// Use [`FromRequest`](crate::request::FromRequest) to obtain a reference to
/// the `Addr` stored in a request:
///
/// ```rust
/// use lumine::{Addr, FromRequest, IntoResponse, Request};
///
/// async fn handler(req: Request) -> impl IntoResponse {
///     let addr = Addr::from_request(&req);
///     format!("Hello, {}!", addr.ip())
/// }
/// ```
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Addr(pub IpAddr);

impl Addr {
    /// Creates a new `Addr` wrapping the given [`IpAddr`].
    pub fn new(ip: IpAddr) -> Self {
        Self(ip)
    }

    /// Returns the wrapped [`IpAddr`] by value.
    pub fn ip(self) -> IpAddr {
        self.0
    }

    /// Returns `true` if the address is an IPv4 address.
    pub fn is_ipv4(self) -> bool {
        self.0.is_ipv4()
    }

    /// Returns `true` if the address is an IPv6 address.
    pub fn is_ipv6(self) -> bool {
        self.0.is_ipv6()
    }

    /// Returns a reference to the wrapped [`IpAddr`].
    pub fn as_ip(&self) -> &IpAddr {
        &self.0
    }
}
