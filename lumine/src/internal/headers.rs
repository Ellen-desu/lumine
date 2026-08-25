//! HTTP headers manipulation module.
//!
//! This module contains helper functions for setting standard HTTP headers
//! on outgoing responses, including date, security, and connection headers.

use std::time::SystemTime;

use http::{HeaderMap, HeaderValue, StatusCode, header, response::Parts};

use crate::body::{Body, DynBody};

/// Sets the appropriate headers for an HTTP response based on its status and body.
///
/// This function populates headers such as `Content-Length`, `Transfer-Encoding`,
/// `Date`, and various default security headers. It also modifies the body if the
/// request method is `HEAD` or if the status code indicates no content.
pub fn set_headers(
    parts: &mut Parts,
    body: &mut DynBody,
    should_close: bool,
    is_method_head: bool,
) {
    let headers = &mut parts.headers;
    headers.reserve(8);

    let status = parts.status;

    set_date(headers);

    set_default_security(headers);

    set_connection(headers, should_close);

    if status.is_informational() || status == StatusCode::NO_CONTENT {
        *body = Body::Empty;
    }

    let not_modified = status == StatusCode::NOT_MODIFIED;

    match &body {
        Body::Empty => {
            if !not_modified {
                headers.insert(header::CONTENT_LENGTH, 0.into());
            }
        }
        Body::Bytes(bytes) => {
            if !not_modified {
                headers.insert(header::CONTENT_LENGTH, bytes.len().into());
            }

            set_content_type(headers, "text/plain");
        }
        Body::Stream(stream) => {
            if let Some(length) = stream.size_hint()
                && !not_modified
            {
                headers.insert(header::CONTENT_LENGTH, length.into());
            } else if !is_method_head {
                headers.insert(
                    header::TRANSFER_ENCODING,
                    HeaderValue::from_static("chunked"),
                );
            }

            if let Some(hints) = stream.headers_hint() {
                hints.iter().for_each(|(key, val)| {
                    headers.entry(key).or_insert(val.clone());
                });
            }

            set_content_type(headers, "application/octet-stream");
        }
    }

    if is_method_head || not_modified {
        headers.remove(header::TRANSFER_ENCODING);

        if not_modified {
            headers.remove(header::CONTENT_LENGTH);
        }

        *body = Body::Empty;
    }
}

/// Sets the `Connection` header on the given `HeaderMap`.
///
/// If `close` is `true`, it sets the value to `close`. Otherwise, it sets it
/// to `keep-alive`.
pub fn set_connection(headers: &mut HeaderMap, close: bool) {
    headers.insert(
        header::CONNECTION,
        HeaderValue::from_static(if close { "close" } else { "keep-alive" }),
    );
}

/// Sets default security headers on the given `HeaderMap`.
///
/// This includes `X-Content-Type-Options: nosniff` and
/// `Referrer-Policy: strict-origin-when-cross-origin`.
pub fn set_default_security(headers: &mut HeaderMap) {
    // Browser compatibility: set X-Content-Type-Options to nosniff to prevent MIME type sniffing
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );

    // Browser compatibility: set Referrer-Policy to strict-origin-when-cross-origin to prevent leaking referrer information
    headers.insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
}

/// Sets the `Date` header on the given `HeaderMap` to the current system time.
pub fn set_date(headers: &mut HeaderMap) {
    headers.insert(
        header::DATE,
        httpdate::fmt_http_date(SystemTime::now())
            .parse()
            .expect("parse string to header should always work"),
    );
}

/// Sets the `Content-Type` header if it is not already present.
pub fn set_content_type(headers: &mut HeaderMap, default: &'static str) {
    headers
        .entry(header::CONTENT_TYPE)
        .or_insert(HeaderValue::from_static(default));
}
