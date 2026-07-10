#[cfg(feature = "date")]
use std::time::SystemTime;

use http::{HeaderMap, HeaderValue, StatusCode, header};

use crate::body::{Body, DynBody};

pub fn set_headers(
    headers: &mut HeaderMap,
    status: StatusCode,
    body: &mut DynBody,
    should_close: bool,
    is_method_head: bool,
) {
    #[cfg(feature = "date")]
    headers.insert(
        header::DATE,
        httpdate::fmt_http_date(SystemTime::now())
            .parse()
            .expect("parse string to header should always work"),
    );

    headers.insert(
        header::CONNECTION,
        HeaderValue::from_static(if should_close { "close" } else { "keep-alive" }),
    );

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

            insert_content_type(headers, "text/plain");
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

            insert_content_type(headers, "application/octet-stream");
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

fn insert_content_type(headers: &mut HeaderMap, default: &'static str) {
    headers
        .entry(header::CONTENT_TYPE)
        .or_insert(HeaderValue::from_static(default));
}
