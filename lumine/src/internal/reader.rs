//! Request reading and parsing module.
//!
//! This module provides functionality to asynchronously read an HTTP request from
//! a TCP stream, parse the request line and headers, and construct a `Request` object.

use crate::{
    application::limits::Limits,
    error::Error,
    internal::{framing::Framing, validator},
    request::{Request, extensions::query::Query},
};
use bytes::{Bytes, BytesMut};
use http::{HeaderMap, HeaderName, HeaderValue, Method, Uri, Version};
use memchr::memmem;
use tokio::io::{AsyncBufRead, AsyncReadExt};

/// This function reads the request line, headers, and body from the stream
/// and constructs a [`Request`] object.
pub async fn read_request<R: AsyncBufRead + Unpin>(
    reader: &mut R,
    limits: &Limits,
) -> Result<Option<(Request, Framing)>, Error> {
    let mut buffer = BytesMut::with_capacity(4096);

    let header_end = loop {
        if let Ok(0) = reader.read_buf(&mut buffer).await {
            return Ok(None);
        }

        if buffer.len() > limits.max_request_size {
            return Err(Error::RequestTooLarge);
        }

        if let Some(pos) = memmem::find(&buffer, b"\r\n\r\n") {
            break pos;
        }
    };

    let request_line_end = memmem::find(&buffer, b"\r\n").ok_or(Error::InvalidRequestLine)?;
    let (method, uri, version, query) = {
        let request_line = &buffer[..request_line_end];

        let first = memchr::memchr(b' ', request_line).ok_or(Error::InvalidRequestLine)?;

        let second = memchr::memchr(b' ', &request_line[(first + 1)..])
            .ok_or(Error::InvalidRequestLine)?
            + first
            + 1;

        let method = Method::from_bytes(&request_line[..first])?;
        let uri = {
            let bytes = &request_line[(first + 1)..second];

            if bytes.len() > (limits.max_path_size + 1 + limits.max_query_size) {
                return Err(Error::UriTooLarge);
            }

            Uri::try_from(bytes)?
        };

        let version = if &request_line[(second + 1)..] == b"HTTP/1.1" {
            Version::HTTP_11
        } else {
            return Err(Error::HttpVersionNotSupported);
        };

        let query = if let Some(query_str) = uri.query() {
            let mut query = Query::with_capacity(8);

            if query_str.len() > limits.max_query_size {
                return Err(Error::QueryTooLarge);
            }

            for (pairs, (key, value)) in form_urlencoded::parse(query_str.as_bytes())
                .into_owned()
                .enumerate()
            {
                if pairs > limits.max_query_count {
                    return Err(Error::QueryTooLarge);
                }

                query.insert(key.into_boxed_str(), value.into_boxed_str());
            }

            query
        } else {
            Query::new()
        };

        (method, uri, version, query)
    };

    let headers = {
        let mut headers = HeaderMap::with_capacity(8);

        let headers_block = &buffer[(request_line_end + 2)..header_end + 2];
        if headers_block.len() > limits.max_headers_size {
            return Err(Error::HeadersTooLarge);
        }

        for (pairs, line) in headers_block.split(|b| *b == b'\n').enumerate() {
            if line.is_empty() {
                break;
            }

            let line = line.strip_suffix(b"\r").ok_or(Error::InvalidHeaders)?;

            let colon = memchr::memchr(b':', line).ok_or(Error::InvalidHeaders)?;

            let key = HeaderName::from_bytes(&line[..colon])?;
            let value = HeaderValue::from_bytes(line[colon + 1..].trim_ascii())?;

            if pairs >= limits.max_headers_count {
                return Err(Error::HeadersTooLarge);
            }

            headers.append(key, value);
        }

        headers
    };

    let framing = validator::validate_headers(&headers)?;

    let body = match framing.content_length {
        Some(0) | None => Bytes::new(),
        Some(content_length) => {
            if content_length > limits.max_body_size {
                return Err(Error::BodyTooLarge);
            }

            let body_start = header_end + 4;
            let buffered_body = &buffer[body_start..];
            let already_read = buffered_body.len();

            let mut body = BytesMut::with_capacity(content_length);

            body.extend_from_slice(buffered_body);

            if content_length - already_read > 0 {
                body.resize(content_length, 0);
                if reader.read_exact(&mut body[already_read..]).await.is_err() {
                    return Ok(None);
                }
            }

            body.freeze()
        }
    };

    let mut request = http::Request::new(body);
    request.extensions_mut().insert(query);

    *request.uri_mut() = uri;
    *request.method_mut() = method;
    *request.version_mut() = version;
    *request.headers_mut() = headers;

    Ok(Some((request, framing)))
}
