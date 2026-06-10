//! Connection handling module.
//!
//! This module provides functionality to handle TCP connections, parse requests,
//! dispatch them to the appropriate route, and write responses back to the stream.

use crate::{
    application::{lumine::Lumine, states::Ready},
    internal::{handler::request::dispatch_request, handler::writer::write_response, parser},
    routing::into_response::IntoResponse,
    types::result::Result,
    utils::default_headers::DefaultHeaders,
};
use http::header::CONNECTION;
use std::{net::TcpStream, sync::Arc};

/// Handles an incoming TCP connection loop.
///
/// This function continuously parses requests from the stream, dispatches them
/// to the application, and writes back the responses. It manages connection
/// closure based on request headers.
pub fn handle_connection(app: Arc<Lumine<Ready>>, stream: TcpStream) -> Result<()> {
    loop {
        let request_result = parser::parse_request(app.limits, &stream);

        let (request, client_wants_close) = match request_result {
            Ok(Some(request)) => {
                let wants_close = request
                    .headers()
                    .get(CONNECTION)
                    .map(|v| v.as_bytes().eq_ignore_ascii_case(b"close"))
                    .unwrap_or(false);

                (request, wants_close)
            }
            // Client disconnected
            Ok(None) => break,
            Err(error) => {
                write_response(error.into_response()?, &stream)?;

                break;
            }
        };

        let (response, final_close) =
            dispatch_request(request, &app)?.set_default_headers(client_wants_close)?;

        write_response(response, &stream)?;

        if final_close {
            break;
        }
    }

    Ok(())
}
