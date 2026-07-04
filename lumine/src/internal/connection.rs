//! Connection handling module.
//!
//! This module provides functionality to handle TCP connections, parse requests,
//! dispatch them to the appropriate route, and write responses back to the stream.

use crate::{
    application::{lumine::Lumine, states::Ready},
    internal::{dispatch, headers, reader, writer},
    response::into_response::IntoResponse,
};
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncWrite};

/// Handles an incoming TCP connection loop.
///
/// This function continuously parses requests from the stream, dispatches them
/// to the application, and writes back the responses. It manages connection
/// closure based on request headers.
pub async fn handle_connection<Rw: AsyncRead + AsyncWrite + Unpin>(
    app: Arc<Lumine<Ready>>,
    mut stream: Rw,
) {
    let timeouts = &app.timeouts;

    loop {
        let request_result = match tokio::time::timeout(
            timeouts.request_read,
            reader::read_request(&mut stream, &app.limits),
        )
        .await
        {
            Ok(request_result) => request_result,
            _ => break,
        };

        let (request, framing) = match request_result {
            Ok(Some((request, framing))) => (request, framing),
            // Client disconnected
            Ok(None) => break,
            Err(error) => {
                let _ = writer::write_response(error.into_response(), &mut stream, timeouts).await;

                break;
            }
        };

        let (mut parts, body) = dispatch::dispatch_request(request, &app).await.into_parts();

        let should_close = framing.connection.is_close()
            || parts.status.is_server_error()
            || parts.status.is_client_error();

        headers::set_headers(&mut parts.headers, &body, should_close);

        let response = http::Response::from_parts(parts, body);
        if writer::write_response(response, &mut stream, timeouts)
            .await
            .is_err()
        {
            break;
        }

        if should_close {
            break;
        }
    }
}
