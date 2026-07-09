use crate::{
    application::limits::Limits,
    error::Error,
    internal::{framing::Framing, parser, validator},
    request::Request,
};
use http::HeaderMap;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, BufReader};

/// This function reads the request line, headers, and body from the stream
/// and constructs a [`Request`] object.
pub async fn read_request<R: AsyncRead + Unpin>(
    stream: &mut R,
    limits: &Limits,
) -> Result<Option<(Request, Framing)>, Error> {
    let mut reader = BufReader::new(stream);
    let mut buffer = String::new();

    // Request line
    if let Ok(0) = reader.read_line(&mut buffer).await {
        return Ok(None);
    }

    let (method, uri, version, query) = parser::parse_request_line(&buffer, limits)?;

    // Headers
    let mut headers = HeaderMap::with_capacity(16);
    loop {
        buffer.clear();
        if let Ok(0) = reader.read_line(&mut buffer).await {
            return Ok(None);
        }

        if buffer.trim().is_empty() {
            break;
        }

        if headers.len() >= limits.max_headers_count {
            return Err(Error::HeadersTooLarge);
        }

        let (key, value) = parser::parse_header(&buffer, limits)?;

        headers.append(key, value);
    }

    let framing = validator::validate_headers(&headers)?;

    let body = match framing.content_length {
        Some(0) | None => Vec::new(),
        Some(content_length) => {
            if content_length > limits.max_body_size {
                return Err(Error::BodyTooLarge);
            }

            let mut body = vec![0u8; content_length];
            if reader.read_exact(&mut body).await.is_err() {
                return Ok(None);
            }

            body
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
