use crate::{
    application::{lumine::Lumine, states::Ready},
    body::Body,
    internal::parser,
    middleware::next::Next,
    routing::into_response::IntoResponse,
    stream::Stream,
    types::{request::Request, response::Response, result::Result},
    utils::default_headers::DefaultHeaders,
};
use http::{StatusCode, header::CONNECTION};
use std::{
    io::{BufWriter, Write},
    net::TcpStream,
    panic::{self, AssertUnwindSafe},
    sync::Arc,
};

pub fn handle_client(app: Arc<Lumine<Ready>>, stream: TcpStream) -> Result<()> {
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
            handle_request(request, &app)?.set_default_headers(client_wants_close)?;

        write_response(response, &stream)?;

        if final_close {
            break;
        }
    }

    Ok(())
}

pub fn handle_request(mut request: Request, app: &Arc<Lumine<Ready>>) -> Result<Response> {
    let response = match app.get_route(request.uri()) {
        Some((route, params)) => {
            request.extensions_mut().insert(params);

            let mut chain = Vec::new();

            // Choose between route or global middleware which takes precedence
            let iter = if route.route_middleware_first() {
                route.middlewares().iter().chain(app.middlewares.iter())
            } else {
                app.middlewares.iter().chain(route.middlewares())
            }
            .map(|b| b.as_ref());

            chain.extend(iter);

            let next = Next {
                middlewares: &chain,
                route,
            };

            // Start the middleware chain and catch the panic to prevent app from crash
            match panic::catch_unwind(AssertUnwindSafe(|| next.run(request).unwrap())) {
                Ok(response) => response,
                _ => http::Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::Empty)?,
            }
        }
        _ => http::Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::Empty)?,
    };

    Ok(response)
}

pub fn write_response(response: Response, stream: &TcpStream) -> Result<()> {
    let mut writer = BufWriter::new(stream);

    // Status line
    let version = response.version();
    let status = response.status();

    // Write the status line to the stream
    write!(writer, "{version:?} {status}\r\n")?;

    // Headers
    for (name, value) in response.headers() {
        // Write each header to the stream
        write!(writer, "{name}: {}\r\n", value.to_str().unwrap_or_default())?;
    }

    // End of headers
    writer.write_all(b"\r\n")?;

    // Write body to the stream
    match response.into_body() {
        Body::Bytes(bytes) => {
            writer.write_all(&bytes)?;
        }
        Body::Stream(stream) => {
            if stream.size_hint().is_some() {
                write_body_static(stream, &mut writer)?;
            } else {
                write_body_chunked(stream, &mut writer)?;
            }
        }
        _ => {}
    };

    writer.flush()?;

    Ok(())
}

pub fn write_body_chunked<S: Stream, W: Write>(mut bytes_stream: S, writer: &mut W) -> Result<()> {
    let mut buffer = [0u8; 8192];

    loop {
        let n = bytes_stream.next_chunk(&mut buffer)?;
        if n == 0 {
            break;
        }

        write!(writer, "{n:x}\r\n")?;
        writer.write_all(&buffer[..n])?;
        writer.write_all(b"\r\n")?;
    }

    writer.write_all(b"0\r\n\r\n")?;

    Ok(())
}

pub fn write_body_static<S: Stream, W: Write>(mut bytes_stream: S, writer: &mut W) -> Result<()> {
    let mut buffer = [0u8; 8192];

    loop {
        let n = bytes_stream.next_chunk(&mut buffer)?;
        if n == 0 {
            break;
        }

        writer.write_all(&buffer[..n])?;
    }

    Ok(())
}
