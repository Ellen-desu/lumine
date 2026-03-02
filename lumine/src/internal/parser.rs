use crate::{
    error::Error,
    types::{body::Body, result::Result},
};
use http::{HeaderMap, HeaderName, HeaderValue, Method, Uri, Version, header::CONTENT_LENGTH};
use std::{
    io::{BufReader, Read},
    net::TcpStream,
    str::FromStr,
};

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
pub(crate) fn parse_header(line: &str) -> Result<(HeaderName, HeaderValue)> {
    let (k, v) = line
        .split_once(": ")
        .map(|(k, v)| (k.to_lowercase(), v.trim()))
        .unwrap_or_default();

    let header_name = HeaderName::from_lowercase(k.as_bytes())?;
    let header_value = HeaderValue::from_str(v)?;

    Ok((header_name, header_value))
}

pub(crate) fn parse_body(headers: &HeaderMap, reader: &mut BufReader<&TcpStream>) -> Result<Body> {
    let content_len = match headers.get(CONTENT_LENGTH) {
        Some(header_value) => {
            let len_str = header_value.to_str().map_err(|_| Error::Parser)?;

            len_str.parse::<usize>().map_err(|_| Error::Parser)?
        }
        _ => 0,
    };

    let mut body = vec![0u8; content_len];
    reader.read_exact(&mut body)?;

    Ok(body)
}
