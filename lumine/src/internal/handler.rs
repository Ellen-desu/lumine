use crate::{
    application::{lumine::Lumine, states::Ready},
    error::Error,
    internal::parser,
    middleware::next::Next,
    types::{body::Body, request::Request, response::Response, result::Result},
};
use chrono::Utc;
use http::{
    HeaderValue, StatusCode,
    header::{CONNECTION, CONTENT_LENGTH, CONTENT_TYPE, DATE},
};
use std::{
    io::{BufWriter, Write},
    net::TcpStream,
    panic::{self, AssertUnwindSafe},
    sync::Arc,
};

pub(crate) fn handle_client(app: Arc<Lumine<Ready>>, stream: TcpStream) -> Result<()> {
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
                let mut response = http::Response::builder()
                    .status(match error {
                        Error::UriTooLarge | Error::QueryTooLarge => StatusCode::URI_TOO_LONG,
                        Error::HeadersTooLarge => StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE,
                        Error::BodyTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
                        _ => StatusCode::BAD_REQUEST,
                    })
                    .body(Body::default())?;

                response
                    .headers_mut()
                    .append(CONNECTION, HeaderValue::from_static("close"));

                write_response(response, &stream)?;

                break;
            }
        };

        let mut response = handle_request(request, &app)?;

        let server_wants_close = response.status().is_server_error();

        let final_close = client_wants_close || server_wants_close;

        set_default_header(&mut response, final_close)?;

        write_response(response, &stream)?;

        if final_close {
            break;
        }
    }

    Ok(())
}


fn handle_request(mut request: Request, app: &Arc<Lumine<Ready>>) -> Result<Response> {
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
                    .body(Body::default())?,
            }
        }
        _ => http::Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::default())?,
    };

    Ok(response)
}

fn write_response(response: Response, stream: &TcpStream) -> Result<()> {
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
    write!(writer, "\r\n")?;

    // Write body to the stream
    writer.write_all(response.body())?;

    writer.flush()?;

    Ok(())
}

fn set_default_header(response: &mut Response, should_close: bool) -> Result<()> {
    let content_length = response.body().len();
    let now = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();

    let headers = response.headers_mut();

    if let None = headers.get(CONTENT_TYPE)
        && content_length != 0
    {
        headers.append(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
    }

    headers.append(DATE, HeaderValue::from_str(&now)?);
    headers.append(CONTENT_LENGTH, HeaderValue::from(content_length));

    if response.headers().contains_key(CONNECTION) {
        return Ok(());
    }

    let (key, value) = if should_close {
        (CONNECTION, HeaderValue::from_static("close"))
    } else {
        (CONNECTION, HeaderValue::from_static("keep-alive"))
    };

    response.headers_mut().append(key, value);

    Ok(())
}
