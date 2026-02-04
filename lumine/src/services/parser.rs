use crate::{
    error::Error,
    types::{body::Body, result::Result},
};
use http::{HeaderMap, HeaderName, HeaderValue, Method, Uri, Version};
use std::{
    io::{BufReader, Read},
    net::TcpStream,
    str::FromStr,
};

pub(crate) fn parse_request_line(line: &str) -> Result<(Method, Uri, Version)> {
    let mut split = line.split_whitespace();

    let method = Method::from_str(split.next().ok_or(Error::InvalidRequestLine)?)?;

    let uri = Uri::from_str(split.next().ok_or(Error::InvalidRequestLine)?)?;

    let version_str = split.next().ok_or(Error::InvalidRequestLine)?;
    let version = match version_str {
        "HTTP/0.9" => Version::HTTP_09,
        "HTTP/1.0" => Version::HTTP_10,
        "HTTP/1.1" => Version::HTTP_11,
        "HTTP/2" => Version::HTTP_2,
        "HTTP/3" => Version::HTTP_3,
        _ => return Err(Error::InvalidVersion(version_str.into())),
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
    let content_len = match headers.get("content-length") {
        Some(header_value) => {
            let len_str = header_value
                .to_str()
                .map_err(|_| Error::InvalidBody("Failed to parse Content-Length header to str"))?;

            len_str
                .parse::<usize>()
                .map_err(|_| Error::InvalidBody("Failed to parse Content-Length header to int"))?
        }
        _ => 0,
    };

    let mut body = vec![0u8; content_len];
    reader.read_exact(&mut body)?;

    Ok(body)
}
