use std::{fmt, marker::PhantomData};

use crate::application::states::{Builder, Ready};
use http::{Method, StatusCode, Uri};

/// Represents a connected HTTP client inside **Lumine**.
///
/// `Client` uses a compile-time state system to ensure correct API usage.
/// During the `Builder` state, fields may be modified through methods.
/// Once built, it transitions into the `Ready` state.
pub struct Client<State = Builder> {
    method: Method,
    status: StatusCode,
    url: Uri,
    _state: PhantomData<State>,
}

impl Client {
    pub(crate) fn builder() -> Self {
        Client {
            method: Method::default(),
            status: StatusCode::default(),
            url: Uri::default(),
            _state: PhantomData::<Builder>,
        }
    }
}

impl Client<Builder> {
    pub(crate) fn build(self) -> Client<Ready> {
        Client {
            method: self.method,
            status: self.status,
            url: self.url,
            _state: PhantomData::<Ready>,
        }
    }
    pub(crate) fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }
    pub(crate) fn status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }
    pub(crate) fn url(mut self, url: Uri) -> Self {
        self.url = url;
        self
    }
}

impl Client<Ready> {
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
}

impl fmt::Debug for Client<Ready> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("method", &self.method)
            .field("status", &self.status())
            .field("url", &self.url())
            .finish()
    }
}
