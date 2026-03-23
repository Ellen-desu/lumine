use crate::{
    application::{client::Client, lumine::Lumine, states::Ready},
    internal::parser,
    middleware::next::Next,
    types::{body::Body, request::Request, response::Response, result::Result},
};
use chrono::Utc;
use http::{
    HeaderMap, HeaderValue, StatusCode,
    header::{CONNECTION, CONTENT_LENGTH, CONTENT_TYPE, DATE},
};
use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpStream,
    panic::{self, AssertUnwindSafe},
    sync::{Arc, mpsc::Sender},
};

pub(crate) fn handle_client(
    app: Arc<Lumine<Ready>>,
    stream: TcpStream,
    tx: Arc<Sender<Client>>,
) -> Result<()> {
    loop {
        let mut client = Client::default();
        let request_result = read_request(&stream);

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
            Err(_) => {
                let mut response = http::Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::default())?;

                response
                    .headers_mut()
                    .append(CONNECTION, HeaderValue::from_static("close"));

                write_response(response, &stream)?;

                break;
            }
        };

        client.method = request.method().clone();
        client.url = request.uri().clone();

        let mut response = handle_request(request, &app)?;

        client.status = response.status();
        let _ = tx.send(client);

        let server_wants_close = should_server_close(&response);

        let final_close = client_wants_close || server_wants_close;

        set_connection_header(&mut response, final_close)?;
        set_default_header(&mut response)?;

        write_response(response, &stream)?;

        if final_close {
            break;
        }
    }

    Ok(())
}

fn handle_request(mut request: Request, app: &Arc<Lumine<Ready>>) -> Result<Response> {
    let response = match app.get_route(request.uri()) {
        Some((route, params, query)) => {
            if request.body().len() > app.max_body() {
                http::Response::builder()
                    .status(StatusCode::PAYLOAD_TOO_LARGE)
                    .body(Body::default())?
            } else {
                let ext = request.extensions_mut();
                ext.insert(params);
                ext.insert(query);

                let mut chain = Vec::new();

                let iter = if route.route_middleware_first() {
                    route.middlewares().iter().chain(app.middlewares())
                } else {
                    app.middlewares().iter().chain(route.middlewares())
                }
                .map(|b| b.as_ref());

                chain.extend(iter);

                let next = Next {
                    middlewares: &chain,
                    route,
                };

                match panic::catch_unwind(AssertUnwindSafe(|| next.run(request).unwrap())) {
                    Ok(response) => response,
                    _ => http::Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::default())?,
                }
            }
        }
        _ => http::Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::default())?,
    };

    Ok(response)
}

fn read_request(stream: &TcpStream) -> Result<Option<Request>> {
    let mut reader = BufReader::new(stream);

    // Request line
    let mut raw_request_line = String::new();
    if let Ok(0) = reader.read_line(&mut raw_request_line) {
        return Ok(None);
    }

    let (method, uri, version) = parser::parse_request_line(&raw_request_line)?;

    // Headers
    let mut headers = HeaderMap::new();
    loop {
        let mut header_line = String::new();
        if let Ok(0) = reader.read_line(&mut header_line) {
            return Ok(None);
        }

        if header_line.trim().is_empty() {
            break;
        }

        let (name, value) = parser::parse_header(&header_line)?;

        headers.append(name, value);
    }

    let body = parser::parse_body(&headers, &mut reader)?;

    let mut builder = http::Request::builder()
        .method(method)
        .uri(uri)
        .version(version);

    for (k, v) in headers.iter() {
        builder = builder.header(k, v);
    }

    Ok(Some(builder.body(body)?))
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

fn set_default_header(response: &mut Response) -> Result<()> {
    let content_length = response.body().len();
    let now = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();

    let header_mut = response.headers_mut();

    if let None = header_mut.get(CONTENT_TYPE)
        && content_length != 0
    {
        header_mut.append(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
    }

    header_mut.append(DATE, HeaderValue::from_str(&now)?);
    header_mut.append(CONTENT_LENGTH, HeaderValue::from(content_length));

    Ok(())
}

fn should_server_close(response: &Response) -> bool {
    response.status().is_server_error()
}

fn set_connection_header(response: &mut Response, should_close: bool) -> Result<()> {
    if response.headers().contains_key(CONNECTION) {
        return Ok(());
    }

    if should_close {
        response
            .headers_mut()
            .insert(CONNECTION, HeaderValue::from_static("close"));
    } else {
        response
            .headers_mut()
            .insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    }

    Ok(())
}
