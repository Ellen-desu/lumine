use lumine::{error::Error, prelude::*, response::IntoResponse};

#[test]
fn into_response_unit() {
    let response = ().into_response();
    assert_eq!(response.status(), StatusCode::OK);
    assert!(response.body().is_empty());
}

#[test]
fn into_response_status_code() {
    let response = StatusCode::CREATED.into_response();
    assert_eq!(response.status(), StatusCode::CREATED);
    assert!(response.body().is_empty());
}

#[test]
fn into_response_body() {
    let response = "hello".into_response();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.body().as_bytes(), Some(b"hello".as_ref()));
}

#[test]
fn into_response_tuple_status_body() {
    let response = (StatusCode::CREATED, "hello").into_response();
    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(response.body().as_bytes(), Some(b"hello".as_ref()));
}

#[test]
fn into_response_tuple_headers_body() {
    let mut headers = HeaderMap::new();
    headers.insert("x-custom", HeaderValue::from_static("test"));
    let response = (headers, "hello").into_response();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("x-custom"),
        Some(&HeaderValue::from_static("test"))
    );
    assert_eq!(response.body().as_bytes(), Some(b"hello".as_ref()));
}

#[test]
fn into_response_tuple_status_headers_body() {
    let mut headers = HeaderMap::new();
    headers.insert("x-custom", HeaderValue::from_static("test"));
    let response = (StatusCode::ACCEPTED, headers, "hello").into_response();

    assert_eq!(response.status(), StatusCode::ACCEPTED);
    assert_eq!(
        response.headers().get("x-custom"),
        Some(&HeaderValue::from_static("test"))
    );
    assert_eq!(response.body().as_bytes(), Some(b"hello".as_ref()));
}

#[test]
fn into_response_result() {
    let ok: Result<&'static str, StatusCode> = Ok("success");
    let response_ok = ok.into_response();
    assert_eq!(response_ok.status(), StatusCode::OK);
    assert_eq!(response_ok.body().as_bytes(), Some(b"success".as_ref()));

    let err: Result<&'static str, StatusCode> = Err(StatusCode::NOT_FOUND);
    let response_err = err.into_response();
    assert_eq!(response_err.status(), StatusCode::NOT_FOUND);
    assert!(response_err.body().is_empty());
}

#[test]
fn into_response_error_mappings() {
    let mappings = vec![
        (Error::UriTooLarge, StatusCode::URI_TOO_LONG),
        (Error::BodyTooLarge, StatusCode::PAYLOAD_TOO_LARGE),
        (
            Error::HeadersTooLarge,
            StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE,
        ),
        (Error::QueryTooLarge, StatusCode::URI_TOO_LONG),
        (
            Error::HttpVersionNotSupported,
            StatusCode::HTTP_VERSION_NOT_SUPPORTED,
        ),
        (Error::InvalidRequestLine, StatusCode::BAD_REQUEST),
        (Error::InvalidHeaders, StatusCode::BAD_REQUEST),
        (Error::RequestTooLarge, StatusCode::BAD_REQUEST),
        (Error::TooManyConnections, StatusCode::SERVICE_UNAVAILABLE),
        (Error::RequestTimeout, StatusCode::REQUEST_TIMEOUT),
        (Error::Unimplemented, StatusCode::NOT_IMPLEMENTED),
    ];

    for (error, expected_status) in mappings {
        let response = error.into_response();
        assert_eq!(response.status(), expected_status);
        assert_eq!(
            response.headers().get(http::header::CONNECTION),
            Some(&HeaderValue::from_static("close"))
        );
        assert!(response.body().is_empty());
    }
}
