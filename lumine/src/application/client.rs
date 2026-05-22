use std::net::IpAddr;

use http::{Method, StatusCode, Uri};

/// Represents a connected HTTP client inside **Lumine**.
#[derive(Debug)]
pub struct Client {
    pub(crate) method: Method,
    pub(crate) status: StatusCode,
    pub(crate) ip: IpAddr,
    pub(crate) url: Uri,
}

impl Client {
    /// Returns the HTTP status code associated with this client.
    pub fn status(&self) -> &StatusCode {
        &self.status
    }
    /// Returns the requested URI of this client.
    pub fn url(&self) -> &Uri {
        &self.url
    }
    /// Returns the HTTP method used by this client.
    pub fn method(&self) -> &Method {
        &self.method
    }
    pub fn ip(&self) -> &IpAddr {
        &self.ip
    }
}
