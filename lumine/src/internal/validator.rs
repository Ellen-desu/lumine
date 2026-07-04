use crate::{
    error::Error,
    internal::framing::{Connection, Framing},
    routing::{route_service::RouteService, segment::Segment},
};
use http::{HeaderMap, header};
use std::sync::Arc;

/// Validates the headers of an HTTP request and returns a [`Framing`] instance.
pub fn validate_headers(headers: &HeaderMap) -> Result<Framing, Error> {
    // RFC 9112: The HOST header field is required
    if !headers.contains_key(header::HOST) {
        return Err(Error::InvalidHeaders);
    }

    let has_content_length = headers.contains_key(header::CONTENT_LENGTH);
    let has_transfer_encoding = headers.contains_key(header::TRANSFER_ENCODING);

    // RFC 9112: Do not accept Content-Length + Transfer-Encoding
    if has_content_length && has_transfer_encoding {
        return Err(Error::InvalidHeaders);
    }

    // Unsupported Transfer-Encoding
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

#[doc(hidden)]
pub fn check_route_duplicates(routes: &[Arc<dyn RouteService>], segments: &[Segment]) {
    if routes.iter().any(|r| r.is_duplicated(segments)) {
        panic!("Conflicting routes");
    }
}
