use lumine::{internal::parser::parse_request_line, prelude::*};

#[test]
fn normal_line() {
    let line = "GET / HTTP/1.1";
    let result = parse_request_line(line, &Limits::default());
    assert!(result.is_ok());

    let (method, uri, version, query) = result.unwrap();
    assert_eq!(method, Method::GET);
    assert_eq!(uri.path(), "/");
    assert_eq!(version, Version::HTTP_11);
    assert!(query.is_empty());
}

#[test]
fn multiple_queries() {
    let line = "GET /?a=test&q=test2&q=test1 HTTP/1.1";
    let result = parse_request_line(line, &Limits::default());
    assert!(result.is_ok());

    let (method, uri, version, query) = result.unwrap();
    assert_eq!(method, Method::GET);
    assert_eq!(uri.path(), "/");
    assert_eq!(version, Version::HTTP_11);

    assert_eq!(
        *query,
        vec![
            (
                "a".to_string().into_boxed_str(),
                vec!["test".to_string().into_boxed_str()]
            ),
            (
                "q".to_string().into_boxed_str(),
                vec![
                    "test2".to_string().into_boxed_str(),
                    "test1".to_string().into_boxed_str()
                ]
            )
        ]
    );
}

#[test]
fn unsupported_version() {
    let line = "GET / HTTP/2.0";
    let result = parse_request_line(line, &Limits::default());
    assert!(result.is_err());
}

#[test]
fn additional_keyword() {
    let line = "GET / HTTP/1.1 ERR";
    let result = parse_request_line(line, &Limits::default());
    assert!(result.is_err());
}

#[test]
fn overflow_path_size() {
    let line = "GET /users HTTP/1.1";
    let result = parse_request_line(
        line,
        &Limits {
            max_path_size: 5,
            ..Default::default()
        },
    );
    assert!(result.is_err());
}

#[test]
fn overflow_query_size() {
    let line = "GET /?a=b HTTP/1.1";
    let result = parse_request_line(
        line,
        &Limits {
            max_query_size: 2,
            ..Default::default()
        },
    );
    assert!(result.is_err());
}

#[test]
fn overflow_query_keys_count() {
    let line = "GET /?a=a&b=b HTTP/1.1";
    let result = parse_request_line(
        line,
        &Limits {
            max_query_count: 1,
            ..Default::default()
        },
    );
    assert!(result.is_err());
}
