use crate::{
    application::{client::Client, lumine::Lumine, states::Ready},
    internal::parser,
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
    let mut client = Client::default();
    let request_result = read_request(&stream);

    let mut response = match request_result {
        Ok(Some(request)) => {
            client.method = request.method().clone();
            client.url = request.uri().clone();

            handle_request(request, &app)?
        }
        // Client disconected
        Ok(None) => return Ok(()),
        _ => http::Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::default())?,
    };

    client.status = response.status();
    let _ = tx.send(client);

    set_default_header(&mut response)?;

    write_response(response, &stream)?;

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

                match panic::catch_unwind(AssertUnwindSafe(|| route.call(request).unwrap())) {
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
    header_mut.append(CONNECTION, HeaderValue::from_static("keep-alive"));

    Ok(())
}
