//! HTTP request parsing module.
//!
//! This module provides functionality for parsing HTTP request lines, and headers.
//! For body, we just read the raw bytes and pass them along.

use crate::{
    application::limits::Limits, error::Error, request::query::Query, routing::segment::Segment,
};
use http::{HeaderName, HeaderValue, Method, Uri, Version};
use std::str::FromStr;

/// Parses the HTTP request line.
///
/// This function extracts the method, URI, HTTP version, and query parameters
/// from the first line of the HTTP request.
pub fn parse_request_line(
    line: &str,
    limits: &Limits,
) -> Result<(Method, Uri, Version, Query), Error> {
    let mut parts = line.split_whitespace();

    let method = Method::from_str(parts.next().ok_or(Error::InvalidRequestLine)?)?;

    let raw_uri = parts.next().ok_or(Error::InvalidRequestLine)?;
    if raw_uri.len() > limits.max_path_size + limits.max_query_size + 1 {
        // +1 for the query string delimiter '?'
        return Err(Error::UriTooLarge);
    }

    let uri = Uri::from_str(raw_uri)?;
    if uri.path().len() > limits.max_path_size {
        return Err(Error::UriTooLarge);
    }

    // Lumine is only supported on HTTP/1.1. You can use reverse proxy(e.g. Nginx, or Caddy) to support other versions.
    let version = if parts.next().ok_or(Error::InvalidRequestLine)? == "HTTP/1.1" {
        Version::HTTP_11
    } else {
        return Err(Error::HttpVersionNotSupported);
    };

    // The request line should only contain the method, URI, and version.
    if parts.next().is_some() {
        return Err(Error::InvalidRequestLine);
    }

    let mut query = Query::with_capacity(8);
    let mut pairs = 0;

    if let Some(query_str) = uri.query() {
        if query_str.len() > limits.max_query_size {
            return Err(Error::QueryTooLarge);
        }

        for (key, value) in form_urlencoded::parse(query_str.as_bytes()).into_owned() {
            pairs += 1;

            if pairs > limits.max_query_count {
                return Err(Error::QueryTooLarge);
            }

            query.insert(key.into_boxed_str(), value.into_boxed_str());
        }
    }

    Ok((method, uri, version, query))
}

/// Parses a single HTTP header line.
///
/// This function splits the line into key-value pair and returns a `HeaderName`
/// and `HeaderValue`.
pub fn parse_header(header: &str, limits: &Limits) -> Result<(HeaderName, HeaderValue), Error> {
    if header.len() > limits.max_headers_size {
        return Err(Error::HeadersTooLarge);
    }

    let (key, value) = header.split_once(":").ok_or(Error::InvalidHeaders)?;

    let header_name = HeaderName::from_str(key)?;
    let header_value = HeaderValue::from_str(value.trim())?;

    Ok((header_name, header_value))
}

#[allow(clippy::panic)]
pub fn parse_path(path: &'static str, limits: &Limits) -> Vec<Segment> {
    if path.len() > limits.max_path_size {
        panic!("Path too large: {} > {}", path.len(), limits.max_path_size);
    }

    if path == "/" {
        return Vec::new();
    }

    if !path.starts_with('/') || path.ends_with('/') {
        panic!("Path must start with a slash and not end with a slash");
    }

    let mut segments = Vec::with_capacity(limits.max_segments_count);

    path.split('/').skip(1).for_each(|s| {
        if segments.len() >= limits.max_segments_count {
            panic!("Path segment count exceeds limit");
        }

        match s {
            ":" => {
                panic!("Path parameter must be followed by a name");
            }
            _ => {
                if segments.last() == Some(&Segment::Wildcard) {
                    panic!("Wildcard segment must be the last segment");
                }

                if s.is_empty() {
                    panic!("Path segment must not be empty");
                }

                if segments.contains(&Segment::Param(s.trim_start_matches(':'))) {
                    panic!("Path parameter must be unique");
                }

                if s.len() > limits.max_segment_size {
                    panic!(
                        "Path segment too large: {} > {}",
                        s.len(),
                        limits.max_segment_size
                    );
                }
            }
        }

        segments.push(Segment::from(s));
    });

    segments
}
