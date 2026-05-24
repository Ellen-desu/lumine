use crate::{
    application::limits::Limits,
    error::Error,
    routing::query::Query,
    types::{body::Body, request::Request, result::Result},
};
use http::{HeaderMap, HeaderName, HeaderValue, Method, Uri, Version, header};
use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
    str::FromStr,
};

pub(crate) fn parse_request(limits: Limits, stream: &TcpStream) -> Result<Option<Request>> {
    let mut reader = BufReader::new(stream);
    let mut buffer = String::new();

    // Request line
    if let Ok(0) = reader.read_line(&mut buffer) {
        return Ok(None);
    }

    let (method, uri, version, query) = parse_request_line(
        &buffer,
        limits.max_uri_size,
        limits.max_query_size,
        limits.max_query_count,
    )?;

    // Headers
    let mut headers = HeaderMap::new();
    loop {
        if headers.len() >= limits.max_headers_count {
            return Err(Error::HeadersTooLarge);
        }

        buffer.clear();
        if let Ok(0) = reader.read_line(&mut buffer) {
            return Ok(None);
        }

        if buffer.trim().is_empty() {
            break;
        }

        let (key, value) = parse_headers(&buffer, limits.max_headers_size)?;

        headers.append(key, value);
    }

    let body = match headers.get(header::CONTENT_LENGTH) {
        Some(value) => {
            let content_length = value
                .to_str()
                .map_err(|_| Error::Parser)?
                .parse::<usize>()
                .map_err(|_| Error::Parser)?;

            if content_length > limits.max_body_size {
                return Err(Error::BodyTooLarge);
            }

            parse_body(content_length, &mut reader)?
        }
        _ => Body::new(),
    };

    let mut builder = http::Request::builder()
        .method(method)
        .uri(uri)
        .version(version);

    for (key, value) in headers.iter() {
        builder = builder.header(key, value);
    }

    let mut request = builder.body(body)?;
    request.extensions_mut().insert(query);

    Ok(Some(request))
}

pub(crate) fn parse_request_line(
    line: &str,
    max_uri_size: usize,
    max_query_size: usize,
    max_query_count: usize,
) -> Result<(Method, Uri, Version, Query)> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    let parts_len = parts.len();
    if parts_len != 3 {
        return Err(Error::Parser);
    }

    let method = Method::from_str(parts[0])?;

    let uri = Uri::from_str(parts[1])?;
    if uri.path().len() > max_uri_size {
        return Err(Error::UriTooLarge);
    }

    let version = match parts[2] {
        "HTTP/0.9" => Version::HTTP_09,
        "HTTP/1.0" => Version::HTTP_10,
        "HTTP/1.1" => Version::HTTP_11,
        "HTTP/2" => Version::HTTP_2,
        "HTTP/3" => Version::HTTP_3,
        _ => {
            return Err(Error::Parser);
        }
    };

    let mut query = Query::default();

    if let Some(query_str) = uri.query() {
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()).into_owned() {
            if key.len() > max_query_size
                || value.len() > max_query_size
                || query.len() > max_query_count
            {
                return Err(Error::QueryTooLarge);
            }
            query
                .entry(key.to_string())
                .or_default()
                .push(value.to_string());
        }
    }

    Ok((method, uri, version, query))
}

pub(crate) fn parse_headers(
    header: &str,
    max_headers_size: usize,
) -> Result<(HeaderName, HeaderValue)> {
    if header.len() > max_headers_size {
        return Err(Error::HeadersTooLarge);
    }

    let (key, value) = header
        .split_once(": ")
        .map(|(key, value)| (key.to_lowercase(), value.trim()))
        .unwrap_or_default();

    let header_name = HeaderName::from_lowercase(key.as_bytes())?;
    let header_value = HeaderValue::from_str(value)?;

    Ok((header_name, header_value))
}

pub(crate) fn parse_body<R: BufRead>(length: usize, reader: &mut R) -> Result<Body> {
    let mut body = vec![0u8; length];
    reader.read_exact(&mut body)?;

    Ok(body)
}

#[cfg(feature = "bench")]
pub fn parse_request_line_for_bench(
    line: &str,
    max_uri_size: usize,
    max_query_size: usize,
    max_query_count: usize,
) -> Result<(Method, Uri, Version, Query)> {
    parse_request_line(line, max_uri_size, max_query_size, max_query_count)
}

#[cfg(feature = "bench")]
pub fn parse_headers_for_bench(
    header: &str,
    max_headers_size: usize,
) -> Result<(HeaderName, HeaderValue)> {
    parse_headers(header, max_headers_size)
}

#[cfg(feature = "bench")]
pub fn parse_body_for_bench<R: BufRead>(length: usize, reader: &mut R) -> Result<Body> {
    parse_body(length, reader)
}
