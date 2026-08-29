use bytes::Bytes;
use lumine::{error::Error, internal::reader::read_request, prelude::*};
use tokio::io::BufReader;

#[tokio::test]
async fn get_request() {
    let raw = b"GET /hello HTTP/1.1\r\n\
                 Host: localhost\r\n\
                 \r\n";
    let mut reader = BufReader::new(&raw[..]);

    let (request, _) = read_request(&mut reader, &Limits::default())
        .await
        .unwrap()
        .unwrap();

    assert_eq!(request.method(), Method::GET);
    assert_eq!(request.uri().path(), "/hello");

    let headers = request.headers();
    assert_eq!(headers.len(), 1);
    assert_eq!(
        headers.get("host"),
        Some(&HeaderValue::from_static("localhost"))
    );
}

#[tokio::test]
async fn post_request_with_body() {
    let raw = b"POST /hello HTTP/1.1\r\n\
        Host: localhost\r\n\
        Content-Type: text/plain\r\n\
        Content-Length: 13\r\n\
        \r\n\
        Hello, World!";
    let mut reader = BufReader::new(&raw[..]);

    let (request, _) = read_request(&mut reader, &Limits::default())
        .await
        .unwrap()
        .unwrap();

    assert_eq!(request.method(), Method::POST);
    assert_eq!(request.uri().path(), "/hello");

    let headers = request.headers();
    assert_eq!(headers.len(), 3);
    assert_eq!(
        headers.get("content-type"),
        Some(&HeaderValue::from_static("text/plain"))
    );
    assert_eq!(
        headers.get("content-length"),
        Some(&HeaderValue::from_static("13"))
    );
    assert_eq!(request.body(), &Bytes::from_static(b"Hello, World!"));
}

#[tokio::test]
async fn request_with_query() {
    let raw = b"GET /hello?foo=bar HTTP/1.1\r\n\
                 Host: localhost\r\n\
                 \r\n";
    let mut reader = BufReader::new(&raw[..]);

    let (request, _) = read_request(&mut reader, &Limits::default())
        .await
        .unwrap()
        .unwrap();

    let query = Query::from_request(&request);
    assert_eq!(query.get("foo"), Some("bar"));
    assert!(query.get_all("foo").unwrap().collect::<Vec<_>>() == vec!["bar"]);
}

#[tokio::test]
async fn empty_stream() {
    let raw = b"";
    let mut reader = BufReader::new(&raw[..]);
    let request = read_request(&mut reader, &Limits::default()).await.unwrap();
    assert!(request.is_none());
}

#[tokio::test]
async fn overflow_headers_count() {
    let raw = b"GET /hello HTTP/1.1\r\n\
                 Host: localhost\r\n\
                 User-Agent: Mozilla/5.0\r\n\
                 \r\n";
    let mut reader = BufReader::new(&raw[..]);
    let result = read_request(&mut reader, &Limits::default().max_headers_count(1)).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn overflow_body() {
    let raw = b"POST /hello HTTP/1.1\r\n\
                 Content-Length: 2\r\n\
                 \r\n";
    let mut reader = BufReader::new(&raw[..]);
    let result = read_request(&mut reader, &Limits::default().max_body_size(1)).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn invalid_http_version() {
    let raw = b"GET /hello HTTP/1.0\r\n\
                 Host: localhost\r\n\
                 \r\n";
    let mut reader = BufReader::new(&raw[..]);
    let result = read_request(&mut reader, &Limits::default()).await;
    match result {
        Err(lumine::error::Error::HttpVersionNotSupported) => {}
        _ => panic!("Expected HttpVersionNotSupported"),
    }
}

#[tokio::test]
async fn uri_too_large() {
    let raw = b"GET /verylonguri HTTP/1.1\r\n\
                 Host: localhost\r\n\
                 \r\n";
    let mut reader = BufReader::new(&raw[..]);
    let limits = Limits::default().max_path_size(5).max_query_size(1);
    let result = read_request(&mut reader, &limits).await;
    match result {
        Err(lumine::error::Error::UriTooLarge) => {}
        _ => panic!("Expected UriTooLarge, got: {:?}", result),
    }
}
#[tokio::test]
async fn headers_crlf() {
    let raw = b"GET / HTTP/1.1\r\n\
                 Host: localhost\r\n\
                 X-Test: hello\r\n\
                 \r\n";

    let mut reader = BufReader::new(&raw[..]);

    let (request, _) = read_request(&mut reader, &Limits::default())
        .await
        .unwrap()
        .unwrap();

    assert_eq!(request.headers().len(), 2);
    assert_eq!(
        request.headers().get("host"),
        Some(&HeaderValue::from_static("localhost"))
    );
    assert_eq!(
        request.headers().get("x-test"),
        Some(&HeaderValue::from_static("hello"))
    );
}

#[tokio::test]
async fn headers_bare_lf() {
    let raw = b"GET / HTTP/1.1\r\n\
                 Host: localhost\n\
                 X-Test: hello\r\n\
                 \r\n";

    let mut reader = BufReader::new(&raw[..]);

    let result = read_request(&mut reader, &Limits::default()).await;

    assert!(matches!(result, Err(Error::InvalidHeaders)));
}

#[tokio::test]
async fn headers_bare_cr() {
    let raw = b"GET / HTTP/1.1\r\n\
                 Host: localhost\r\
                 X-Test: hello\r\n\
                 \r\n";

    let mut reader = BufReader::new(&raw[..]);

    let result = read_request(&mut reader, &Limits::default()).await;

    assert!(matches!(result, Err(Error::InvalidHeaders)));
}

#[tokio::test]
async fn headers_crcr() {
    let raw = b"GET / HTTP/1.1\r\n\
                 Host: localhost\r\r\
                 X-Test: hello\r\n\
                 \r\n";

    let mut reader = BufReader::new(&raw[..]);

    let result = read_request(&mut reader, &Limits::default()).await;

    assert!(matches!(result, Err(Error::InvalidHeaders)));
}

#[tokio::test]
async fn headers_lflf() {
    let raw = b"GET / HTTP/1.1\r\n\
                 Host: localhost\n\n\
                 X-Test: hello\r\n\
                 \r\n";

    let mut reader = BufReader::new(&raw[..]);

    let result = read_request(&mut reader, &Limits::default()).await;

    assert!(matches!(result, Err(Error::InvalidHeaders)));
}
