use crate::{
    error::Error,
    internal::framing::{Connection, Framing},
};
use http::{HeaderMap, header};
use std::net::{Ipv4Addr, Ipv6Addr};

/// Validates the headers of an HTTP request and returns a [`Framing`] instance.
pub fn validate_headers(headers: &HeaderMap) -> Result<Framing, Error> {
    let mut host = None;
    for value in headers.get_all(http::header::HOST) {
        if host.is_some() {
            return Err(Error::InvalidHeaders);
        }
        host = Some(value.to_str().map_err(|_| Error::InvalidHeaders)?);
    }

    let Some(host_raw) = host else {
        return Err(Error::InvalidHeaders);
    };

    let (host, port) = if host_raw.starts_with('[') {
        let end_bracket = host_raw.find(']').ok_or(Error::InvalidHeaders)?;
        let (host, remainder) = host_raw.split_at(end_bracket + 1);

        if !remainder.is_empty() && !remainder.starts_with(':') {
            return Err(Error::InvalidHeaders);
        }

        let port = remainder.strip_prefix(':');

        (host, port)
    } else {
        let mut parts = host_raw.splitn(2, ':');

        let host = parts.next().ok_or(Error::InvalidHeaders)?;
        let port = parts.next();

        (host, port)
    };

    if host.starts_with('[') {
        let inner_ip = &host[1..host.len() - 1];
        inner_ip
            .parse::<Ipv6Addr>()
            .map_err(|_| Error::InvalidHeaders)?;
    } else if host.parse::<Ipv4Addr>().is_err() {
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
    }

    if let Some(port) = port {
        port.parse::<u16>().map_err(|_| Error::InvalidHeaders)?;
    }

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

    let mut connection = None;

    for value in headers.get_all(header::CONNECTION) {
        let value = value.to_str().map_err(|_| Error::InvalidHeaders)?;
        for token in value.split(',') {
            match connection {
                None => {
                    if token.trim().eq_ignore_ascii_case("keep-alive") {
                        connection = Some(Connection::KeepAlive);
                    } else if token.trim().eq_ignore_ascii_case("close") {
                        connection = Some(Connection::Close);
                    } else {
                        return Err(Error::InvalidHeaders);
                    }
                }
                Some(_) => return Err(Error::InvalidHeaders),
            }
        }
    }

    Ok(Framing {
        content_length,
        connection: connection.unwrap_or(Connection::KeepAlive),
    })
}
