use http::{HeaderMap, HeaderValue, header};

use crate::body::{Body, DynBody};

pub fn set_headers(headers: &mut HeaderMap, body: &DynBody, should_close: bool) {
    #[cfg(feature = "date")]
    headers.insert(
        header::DATE,
        time::OffsetDateTime::now_utc()
            .format(time::macros::format_description!(
                "[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second] GMT"
            ))
            .expect("valid HTTP format")
            .parse()
            .expect("Parse to header should always work"),
    );

    headers.insert(
        header::CONNECTION,
        HeaderValue::from_static(if should_close { "close" } else { "keep-alive" }),
    );

    match &body {
        Body::Empty => {
            headers.insert(header::CONTENT_LENGTH, 0.into());
        }
        Body::Bytes(bytes) => {
            headers.insert(header::CONTENT_LENGTH, bytes.len().into());

            insert_content_type(headers, "text/plain");
        }
        Body::Stream(stream) => {
            if let Some(length) = stream.size_hint() {
                headers.insert(header::CONTENT_LENGTH, length.into());
            } else {
                headers.insert(
                    header::TRANSFER_ENCODING,
                    HeaderValue::from_static("chunked"),
                );
            }

            if let Some(hints) = stream.headers_hint() {
                for (key, val) in hints {
                    headers.entry(key.clone()).or_insert(val.clone());
                }
            }

            insert_content_type(headers, "application/octet-stream");
        }
    }
}

fn insert_content_type(headers: &mut HeaderMap, default: &'static str) {
    headers
        .entry(header::CONTENT_TYPE)
        .or_insert(HeaderValue::from_static(default));
}
