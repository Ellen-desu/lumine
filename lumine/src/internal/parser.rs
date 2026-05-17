use crate::{
    error::Error,
    types::{body::Body, result::Result},
};
use http::{HeaderName, HeaderValue, Method, Uri, Version};
use std::{io::BufRead, str::FromStr};

pub(crate) fn parse_request_line(line: &str) -> Result<(Method, Uri, Version)> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() > 3 {
        return Err(Error::Parser);
    }

    let method = Method::from_str(parts[0])?;

    let uri = Uri::from_str(parts[1])?;

    let version_str = parts[2];
    let version = match version_str {
        "HTTP/0.9" => Version::HTTP_09,
        "HTTP/1.0" => Version::HTTP_10,
        "HTTP/1.1" => Version::HTTP_11,
        "HTTP/2" => Version::HTTP_2,
        "HTTP/3" => Version::HTTP_3,
        _ => {
            return Err(Error::Parser);
        }
    };

    Ok((method, uri, version))
}
pub(crate) fn parse_headers(header: &str) -> Result<(HeaderName, HeaderValue)> {
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
pub fn parse_request_line_for_bench(line: &str) -> Result<(Method, Uri, Version)> {
    parse_request_line(line)
}

#[cfg(feature = "bench")]
pub fn parse_headers_for_bench(header: &str) -> Result<(HeaderName, HeaderValue)> {
    parse_headers(header)
}
