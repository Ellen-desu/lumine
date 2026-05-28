use crate::{
    attachment::Attachment, body::Body, routing::into_body::IntoBody, types::response::Response,
    utils::set_headers::SetHeaders,
};
use http::{
    HeaderMap, HeaderValue, Result, StatusCode,
    header::{CONTENT_DISPOSITION, CONTENT_TYPE},
};
use std::io::{self, ErrorKind};

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
        http::Response::builder().body(Body::Empty)
    }
}

impl<B> IntoResponse for (StatusCode, B)
where
    B: IntoBody,
{
    fn into_response(self) -> Result<Response> {
        http::Response::builder()
            .status(self.0)
            .body(self.1.into_body())
    }
}

impl<B> IntoResponse for (u16, B)
where
    B: IntoBody,
{
    fn into_response(self) -> Result<Response> {
        let builder = http::Response::builder();

        if !(100..=599).contains(&self.0) {
            return builder
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::Empty);
        }

        builder.status(self.0).body(self.1.into_body())
    }
}

impl IntoResponse for (StatusCode, HeaderMap) {
    fn into_response(self) -> Result<Response> {
        http::Response::builder()
            .status(self.0)
            .headers(&self.1)
            .body(Body::Empty)
    }
}

impl IntoResponse for (u16, HeaderMap) {
    fn into_response(self) -> Result<Response> {
        let builder = http::Response::builder();
        if !(100..=599).contains(&self.0) {
            return http::Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::Empty);
        }

        builder.headers(&self.1).status(self.0).body(Body::Empty)
    }
}

impl<B> IntoResponse for (HeaderMap, B)
where
    B: IntoBody,
{
    fn into_response(self) -> Result<Response> {
        http::Response::builder()
            .headers(&self.0)
            .body(self.1.into_body())
    }
}

impl<B> IntoResponse for (StatusCode, HeaderMap, B)
where
    B: IntoBody,
{
    fn into_response(self) -> Result<Response> {
        http::Response::builder()
            .status(self.0)
            .headers(&self.1)
            .body(self.2.into_body())
    }
}

impl<B> IntoResponse for (u16, HeaderMap, B)
where
    B: IntoBody,
{
    fn into_response(self) -> Result<Response> {
        let builder = http::Response::builder();

        if !(100..=599).contains(&self.0) {
            return builder
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::Empty);
        }

        builder
            .headers(&self.1)
            .status(self.0)
            .body(self.2.into_body())
    }
}

impl IntoResponse for StatusCode {
    fn into_response(self) -> Result<Response> {
        http::Response::builder().status(self).body(Body::Empty)
    }
}

impl<B> IntoResponse for B
where
    B: IntoBody,
{
    fn into_response(self) -> Result<Response> {
        http::Response::builder().body(self.into_body())
    }
}

impl IntoResponse for u16 {
    fn into_response(self) -> Result<Response> {
        let builder = http::Response::builder();
        if !(100..=599).contains(&self) {
            return builder
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::Empty);
        }

        builder.status(self).body(Body::Empty)
    }
}

impl IntoResponse for io::Result<Attachment> {
    fn into_response(self) -> Result<Response> {
        let mut builder = http::Response::builder();

        match self {
            Ok(file) => {
                // Automatically set content type to "application/octet-stream" in `DefaultHeaders::set_default_headers`.
                // So, None value can be ignored.
                if let Some(info) = file.info {
                    builder =
                        builder.header(CONTENT_TYPE, HeaderValue::from_static(info.mime_type()));
                };

                builder = builder.header(
                    CONTENT_DISPOSITION,
                    HeaderValue::from_str(&format!("attachment; filename={}", file.filename))?,
                );

                builder.body(Body::Stream(Box::new(file)))
            }
            Err(error) => {
                let status = match error.kind() {
                    ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    ErrorKind::PermissionDenied => StatusCode::FORBIDDEN,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };

                builder.status(status).body(Body::Empty)
            }
        }
    }
}
