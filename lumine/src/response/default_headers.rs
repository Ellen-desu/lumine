use crate::{body::Body, response::Response};
use http::{HeaderValue, header};

pub(crate) trait DefaultHeaders {
    fn set_default_headers(self, should_close: bool) -> Response;
}

impl DefaultHeaders for Response {
    fn set_default_headers(self, should_close: bool) -> Response {
        let (mut parts, body) = self.into_parts();

        #[cfg(feature = "date")]
        parts.headers.insert(
            header::DATE,
            time::OffsetDateTime::now_utc().format(time::macros::format_description!(
                "[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second] GMT"
            )).expect("valid HTTP format").parse().expect("Parse to header should always work"),
        );

        parts.headers.insert(
            header::CONNECTION,
            HeaderValue::from_static(if should_close { "close" } else { "keep-alive" }),
        );

        match &body {
            Body::Empty => {
                parts.headers.insert(header::CONTENT_LENGTH, 0.into());
            }
            Body::Bytes(bytes) => {
                parts
                    .headers
                    .insert(header::CONTENT_LENGTH, bytes.len().into());
                parts
                    .headers
                    .entry(header::CONTENT_TYPE)
                    .or_insert(HeaderValue::from_static("text/plain"));
            }
            Body::Stream(stream) => {
                if let Some(length) = stream.size_hint() {
                    parts.headers.insert(header::CONTENT_LENGTH, length.into());
                } else {
                    parts.headers.insert(
                        header::TRANSFER_ENCODING,
                        HeaderValue::from_static("chunked"),
                    );
                }

                if let Some(hints) = stream.headers_hint() {
                    for (key, val) in hints {
                        parts.headers.entry(key.clone()).or_insert(val.clone());
                    }
                }

                // Content type fallback
                parts
                    .headers
                    .entry(header::CONTENT_TYPE)
                    .or_insert(HeaderValue::from_static("application/octet-stream"));
            }
        }

        http::Response::from_parts(parts, body)
    }
}
