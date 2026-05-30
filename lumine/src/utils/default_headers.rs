use crate::{
    body::Body,
    types::{response::Response, result::Result},
};
use chrono::Utc;
use http::header::{
    CONNECTION, CONTENT_LENGTH, CONTENT_TYPE, DATE, HeaderValue, TRANSFER_ENCODING,
};

pub(crate) trait DefaultHeaders {
    fn set_default_headers(self, client_wants_close: bool) -> Result<(Response, bool)>;
}

impl DefaultHeaders for Response {
    fn set_default_headers(self, client_wants_close: bool) -> Result<(Response, bool)> {
        let (mut parts, body) = self.into_parts();

        parts.headers.insert(
            DATE,
            HeaderValue::from_str(&Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string())?,
        );

        let final_close = client_wants_close || parts.status.is_server_error();
        let (key, value) = if final_close {
            (CONNECTION, HeaderValue::from_static("close"))
        } else {
            (CONNECTION, HeaderValue::from_static("keep-alive"))
        };

        parts.headers.insert(key, value);

        match &body {
            Body::Empty => {
                parts.headers.insert(CONTENT_LENGTH, 0.into());
            }
            Body::Bytes(bytes) => {
                parts.headers.insert(CONTENT_LENGTH, bytes.len().into());
                parts
                    .headers
                    .entry(CONTENT_TYPE)
                    .or_insert(HeaderValue::from_static("text/plain"));
            }
            Body::Stream(stream) => {
                if let Some(length) = stream.size_hint() {
                    parts.headers.insert(CONTENT_LENGTH, length.into());
                } else {
                    parts
                        .headers
                        .insert(TRANSFER_ENCODING, HeaderValue::from_static("chunked"));
                }

                parts
                    .headers
                    .entry(CONTENT_TYPE)
                    .or_insert(HeaderValue::from_static("application/octet-stream"));
            }
        }

        Ok((http::Response::from_parts(parts, body), final_close))
    }
}
