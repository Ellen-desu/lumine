//! Header validation module.
//!
//! This module provides functionality to validate incoming HTTP request headers
//! and extract necessary framing information such as `Content-Length` and `Connection`.

use crate::{
    error::Error,
    internal::framing::{Connection, Framing},
};
use http::{HeaderMap, header};
use std::net::{Ipv4Addr, Ipv6Addr};

/// Validates the headers of an HTTP request and returns a [`Framing`] instance.
pub fn validate_headers(headers: &HeaderMap) -> Result<Framing, Error> {
    validate_host(headers)?;
    let content_length = parse_content_length(headers)?;
    let connection = parse_connection(headers)?;

    Ok(Framing {
        content_length,
        connection,
    })
}

/// Validates that exactly one well-formed `Host` header is present.
fn validate_host(headers: &HeaderMap) -> Result<(), Error> {
    let mut host = None;
    for value in headers.get_all(header::HOST) {
        if host.is_some() {
            return Err(Error::InvalidHeaders);
        }
        host = Some(value.to_str().map_err(|_| Error::InvalidHeaders)?);
    }

    let host_raw = host.ok_or(Error::InvalidHeaders)?;

    let (host, port) = split_host_port(host_raw)?;
    validate_host_value(host)?;

    if let Some(port) = port {
        port.parse::<u16>().map_err(|_| Error::InvalidHeaders)?;
    }

    Ok(())
}

/// Splits a raw host string into host and optional port components.
///
/// Handles IPv6 bracket notation (e.g. `[::1]:8080`) and plain `host:port`.
fn split_host_port(raw: &str) -> Result<(&str, Option<&str>), Error> {
    if raw.starts_with('[') {
        let end_bracket = raw.find(']').ok_or(Error::InvalidHeaders)?;
        let (host, remainder) = raw.split_at(end_bracket + 1);

        if !remainder.is_empty() && !remainder.starts_with(':') {
            return Err(Error::InvalidHeaders);
        }

        Ok((host, remainder.strip_prefix(':')))
    } else {
        let mut parts = raw.splitn(2, ':');
        let host = parts.next().ok_or(Error::InvalidHeaders)?;
        Ok((host, parts.next()))
    }
}

/// Validates the host component as an IPv6 address, IPv4 address, or DNS hostname.
fn validate_host_value(host: &str) -> Result<(), Error> {
    if host.starts_with('[') {
        let inner_ip = &host[1..host.len() - 1];
        inner_ip
            .parse::<Ipv6Addr>()
            .map_err(|_| Error::InvalidHeaders)?;
    } else if host.parse::<Ipv4Addr>().is_err() {
        validate_hostname(host)?;
    }

    Ok(())
}

/// Validates a DNS hostname according to RFC 952/1123 label rules.
fn validate_hostname(host: &str) -> Result<(), Error> {
    if host.is_empty() || host.len() > 253 {
        return Err(Error::InvalidHeaders);
    }

    for label in host.split('.') {
        if label.is_empty() || label.len() > 63 {
            return Err(Error::InvalidHeaders);
        }

        if label.starts_with('-') || label.ends_with('-') {
            return Err(Error::InvalidHeaders);
        }

        if !label.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return Err(Error::InvalidHeaders);
        }
    }

    Ok(())
}

/// Parses `Content-Length` from the headers, checking for conflicts with `Transfer-Encoding`.
///
/// Returns `Ok(None)` if no `Content-Length` is present.
fn parse_content_length(headers: &HeaderMap) -> Result<Option<usize>, Error> {
    let has_content_length = headers.contains_key(header::CONTENT_LENGTH);
    let has_transfer_encoding = headers.contains_key(header::TRANSFER_ENCODING);

    if has_content_length && has_transfer_encoding {
        return Err(Error::InvalidHeaders);
    }

    if has_transfer_encoding {
        return Err(Error::Unimplemented);
    }

    let mut content_length = None;

    for value in headers.get_all(header::CONTENT_LENGTH) {
        let len = value
            .to_str()
            .map_err(|_| Error::InvalidHeaders)?
            .parse::<usize>()
            .map_err(|_| Error::InvalidHeaders)?;

        match content_length {
            None => content_length = Some(len),
            Some(existing) if existing == len => {}
            Some(_) => {
                return Err(Error::InvalidHeaders);
            }
        }
    }

    Ok(content_length)
}

/// Parses the `Connection` header, defaulting to `KeepAlive` if absent.
fn parse_connection(headers: &HeaderMap) -> Result<Connection, Error> {
    let mut connection = None;

    for value in headers.get_all(header::CONNECTION) {
        let value = value.to_str().map_err(|_| Error::InvalidHeaders)?;
        for token in value.split(',') {
            if connection.is_some() {
                return Err(Error::InvalidHeaders);
            }

            let trimmed = token.trim();
            if trimmed.eq_ignore_ascii_case("keep-alive") {
                connection = Some(Connection::KeepAlive);
            } else if trimmed.eq_ignore_ascii_case("close") {
                connection = Some(Connection::Close);
            } else {
                return Err(Error::InvalidHeaders);
            }
        }
    }

    Ok(connection.unwrap_or(Connection::KeepAlive))
}
