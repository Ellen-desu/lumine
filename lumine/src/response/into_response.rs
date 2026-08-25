//! Response conversion trait.
//!
//! This module provides the [`IntoResponse`] trait, which is the core
//! mechanism for turning handler return values into HTTP responses. It
//! allows handlers to return a wide variety of types, from simple strings
//! to complex tuples containing status codes and headers.

use crate::{
    body::{Body, into_body::IntoBody},
    error::Error,
    internal::headers,
    response::Response,
};
use http::{HeaderMap, StatusCode};

/// Converts a value into an HTTP response.
///
/// `IntoResponse` defines how a value returned from a handler is
/// transformed into a concrete HTTP response
/// (`http::Response<Body>`).
///
/// This trait acts as the final boundary between application logic
/// and the HTTP layer. Handler functions are free to return
/// high-level types, as long as they can be converted into a
/// [`Response`] via `IntoResponse`.
///
/// # Design
///
/// Rather than forcing handlers to manually construct
/// `http::Response<Body>`, Lumine uses `IntoResponse` to provide
/// an ergonomic return interface.
///
/// This allows handlers to return simple values (such as strings)
/// while keeping response construction centralized and consistent.
///
/// # Error handling
///
/// The conversion may fail, in which case an error is returned.
/// This allows response generation to report failures without
/// panicking, and gives the server an opportunity to handle
/// errors gracefully.
///
/// # Usage in handlers
///
/// Any type that implements `IntoResponse` can be returned from
/// a route handler.
///
/// ```rust
/// use lumine::{IntoResponse, Request};
///
/// fn handler(_req: Request) -> impl IntoResponse {
///     "Hello, world!"
/// }
/// ```
///
/// # Notes
///
/// - `IntoResponse` is responsible for producing a complete HTTP
///   response, including status code, headers, and body.
/// - Lower-level details such as writing the response to the
///   network are handled by the server runtime.
pub trait IntoResponse {
    /// Converts the value into an HTTP response.
    fn into_response(self) -> Response;
}

impl IntoResponse for () {
    /// Returns an empty response with a 200 OK status code.
    fn into_response(self) -> Response {
        http::Response::new(Body::Empty)
    }
}

impl<B> IntoResponse for (StatusCode, B)
where
    B: IntoBody,
{
    /// Returns a response with the given status code and body.
    fn into_response(self) -> Response {
        let mut response = http::Response::new(self.1.into_body());
        *response.status_mut() = self.0;
        response
    }
}

impl IntoResponse for (StatusCode, HeaderMap) {
    /// Returns a response with the given status code and headers, with an empty body.
    fn into_response(self) -> Response {
        let mut response = http::Response::new(Body::Empty);
        *response.status_mut() = self.0;
        *response.headers_mut() = self.1;
        response
    }
}

impl<B> IntoResponse for (HeaderMap, B)
where
    B: IntoBody,
{
    /// Returns a response with the given headers and body, with a default 200 OK status code.
    fn into_response(self) -> Response {
        let mut response = http::Response::new(self.1.into_body());
        *response.headers_mut() = self.0;
        response
    }
}

impl<B> IntoResponse for (StatusCode, HeaderMap, B)
where
    B: IntoBody,
{
    /// Returns a response with the given status code, headers, and body.
    fn into_response(self) -> Response {
        let mut response = http::Response::new(self.2.into_body());
        *response.status_mut() = self.0;
        *response.headers_mut() = self.1;
        response
    }
}

impl IntoResponse for StatusCode {
    /// Returns a response with the given status code and an empty body.
    fn into_response(self) -> Response {
        let mut response = http::Response::new(Body::Empty);
        *response.status_mut() = self;
        response
    }
}

impl<B> IntoResponse for B
where
    B: IntoBody,
{
    /// Returns a response with the given body, with a default 200 OK status code.
    fn into_response(self) -> Response {
        http::Response::new(self.into_body())
    }
}

impl IntoResponse for Error {
    /// Converts an application [`Error`] into an HTTP response with an appropriate status code.
    fn into_response(self) -> Response {
        let status = match self {
            Error::UriTooLarge => StatusCode::URI_TOO_LONG,
            Error::BodyTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            Error::HeadersTooLarge => StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE,
            Error::QueryTooLarge => StatusCode::URI_TOO_LONG,
            Error::HttpVersionNotSupported => StatusCode::HTTP_VERSION_NOT_SUPPORTED,
            Error::InvalidRequestLine | Error::InvalidHeaders | Error::RequestTooLarge => {
                StatusCode::BAD_REQUEST
            }
            Error::TooManyConnections => StatusCode::SERVICE_UNAVAILABLE,
            Error::RequestTimeout => StatusCode::REQUEST_TIMEOUT,
            Error::Unimplemented => StatusCode::NOT_IMPLEMENTED,
        };
        let mut response = http::Response::new(Body::Empty);
        *response.status_mut() = status;

        headers::set_connection(response.headers_mut(), true);

        response
    }
}

impl IntoResponse for Response {
    /// Returns the [`Response`] directly.
    fn into_response(self) -> Response {
        self
    }
}

impl<T, E> IntoResponse for Result<T, E>
where
    T: IntoResponse,
    E: IntoResponse,
{
    /// Converts the `Result` into a response. If `Ok`, converts the inner value;
    /// if `Err`, converts the error value.
    fn into_response(self) -> Response {
        match self {
            Ok(value) => value.into_response(),
            Err(error) => error.into_response(),
        }
    }
}
