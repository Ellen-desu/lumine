//! Response conversion trait.
//!
//! This module provides the [`IntoResponse`] trait, which is the core
//! mechanism for turning handler return values into HTTP responses. It
//! allows handlers to return a wide variety of types, from simple strings
//! to complex tuples containing status codes and headers.

use crate::{
    attachment::Attachment,
    body::Body,
    error::Error,
    routing::into_body::IntoBody,
    stream::Stream,
    types::{response::Response, result::Result},
    utils::set_headers::SetHeaders,
};
use http::{
    HeaderMap, HeaderValue, StatusCode,
    header::{CONTENT_DISPOSITION, CONTENT_TYPE},
};
use std::io::ErrorKind;

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
    fn into_response(self) -> Result<Response>;
}

impl IntoResponse for () {
    fn into_response(self) -> Result<Response> {
        Ok(http::Response::builder().body(Body::Empty)?)
    }
}

impl<B> IntoResponse for (StatusCode, B)
where
    B: IntoBody,
{
    fn into_response(self) -> Result<Response> {
        Ok(http::Response::builder()
            .status(self.0)
            .body(self.1.into_body())?)
    }
}

impl<B> IntoResponse for (u16, B)
where
    B: IntoBody,
{
    fn into_response(self) -> Result<Response> {
        let builder = http::Response::builder();

        if !(100..=599).contains(&self.0) {
            return Ok(builder
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::Empty)?);
        }

        Ok(builder.status(self.0).body(self.1.into_body())?)
    }
}

impl IntoResponse for (StatusCode, HeaderMap) {
    fn into_response(self) -> Result<Response> {
        Ok(http::Response::builder()
            .status(self.0)
            .headers(&self.1)
            .body(Body::Empty)?)
    }
}

impl IntoResponse for (u16, HeaderMap) {
    fn into_response(self) -> Result<Response> {
        let builder = http::Response::builder();
        if !(100..=599).contains(&self.0) {
            return Ok(http::Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::Empty)?);
        }

        Ok(builder.headers(&self.1).status(self.0).body(Body::Empty)?)
    }
}

impl<B> IntoResponse for (HeaderMap, B)
where
    B: IntoBody,
{
    fn into_response(self) -> Result<Response> {
        Ok(http::Response::builder()
            .headers(&self.0)
            .body(self.1.into_body())?)
    }
}

impl<B> IntoResponse for (StatusCode, HeaderMap, B)
where
    B: IntoBody,
{
    fn into_response(self) -> Result<Response> {
        Ok(http::Response::builder()
            .status(self.0)
            .headers(&self.1)
            .body(self.2.into_body())?)
    }
}

impl<B> IntoResponse for (u16, HeaderMap, B)
where
    B: IntoBody,
{
    fn into_response(self) -> Result<Response> {
        let builder = http::Response::builder();

        if !(100..=599).contains(&self.0) {
            return Ok(builder
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::Empty)?);
        }

        Ok(builder
            .headers(&self.1)
            .status(self.0)
            .body(self.2.into_body())?)
    }
}

impl IntoResponse for StatusCode {
    fn into_response(self) -> Result<Response> {
        Ok(http::Response::builder().status(self).body(Body::Empty)?)
    }
}

impl<B> IntoResponse for B
where
    B: IntoBody,
{
    fn into_response(self) -> Result<Response> {
        Ok(http::Response::builder().body(self.into_body())?)
    }
}

impl IntoResponse for u16 {
    fn into_response(self) -> Result<Response> {
        let builder = http::Response::builder();
        if !(100..=599).contains(&self) {
            return Ok(builder
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::Empty)?);
        }

        Ok(builder.status(self).body(Body::Empty)?)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Result<Response> {
        Ok(match self {
            Error::Io { source } => {
                let status = match source.kind() {
                    ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    ErrorKind::PermissionDenied => StatusCode::FORBIDDEN,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                http::Response::builder().status(status).body(Body::Empty)
            }
            Error::Parser => http::Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::Empty),
            Error::UriTooLarge => http::Response::builder()
                .status(StatusCode::URI_TOO_LONG)
                .body(Body::Empty),

            Error::BodyTooLarge => http::Response::builder()
                .status(StatusCode::PAYLOAD_TOO_LARGE)
                .body(Body::Empty),
            Error::HeadersTooLarge => http::Response::builder()
                .status(StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE)
                .body(Body::Empty),
            Error::QueryTooLarge => http::Response::builder()
                .status(StatusCode::URI_TOO_LONG)
                .body(Body::Empty),
            _ => http::Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::Empty),
        }?)
    }
}

impl IntoResponse for Attachment {
    fn into_response(self) -> Result<Response> {
        let mut builder = http::Response::builder();

        // Automatically set content type to "application/octet-stream" in `DefaultHeaders::set_default_headers`.
        // So, None value can be ignored.
        if let Some(info) = self.info {
            builder = builder.header(CONTENT_TYPE, HeaderValue::from_static(info.mime_type()));
        };

        builder = builder.header(
            CONTENT_DISPOSITION,
            HeaderValue::from_str(&format!("attachment; filename={}", self.filename))?,
        );

        Ok(builder.body(Body::Stream(Box::new(self) as Box<dyn Stream>))?)
    }
}

impl<T: IntoResponse> IntoResponse for Result<T> {
    fn into_response(self) -> Result<Response> {
        match self {
            Ok(value) => value.into_response(),
            Err(error) => error.into_response(),
        }
    }
}
