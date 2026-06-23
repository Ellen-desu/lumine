use lumine::{internal::parser::parse_header, prelude::*};

#[test]
fn normal_header() {
    let header = "Content-Type: application/json";
    let (key, value) = parse_header(Limits::default(), header).unwrap();

    assert_eq!(key, "Content-Type");
    assert_eq!(value, "application/json");
}

#[test]
fn custom_header() {
    let header = "X-Custom-Header: any";
    let (key, value) = parse_header(Limits::default(), header).unwrap();

    assert_eq!(key, "X-Custom-Header");
    assert_eq!(value, "any");
}

#[test]
fn no_whitespace() {
    let header = "Content-Type:application/json";
    let (key, value) = parse_header(Limits::default(), header).unwrap();

    assert_eq!(key, "Content-Type");
    assert_eq!(value, "application/json");
}

#[test]
fn more_whitespace() {
    let header = "Content-Type:     application/json";
    let (key, value) = parse_header(Limits::default(), header).unwrap();

    assert_eq!(key, "Content-Type");
    assert_eq!(value, "application/json");
}

#[test]
fn whitespace_before_colon() {
    let header = "Content-Type : application/json";
    assert!(parse_header(Limits::default(), header).is_err());
}

#[test]
fn empty_value() {
    let header = "Content-Type:";
    let (key, value) = parse_header(Limits::default(), header).unwrap();

    assert_eq!(key, "Content-Type");
    assert_eq!(value, "");
}

#[test]
fn overflow_header() {
    let kv = "a".repeat(16 * 1024); // 16 KB
    assert!(parse_header(Limits::default(), &format!("{kv}: {kv}")).is_err());
}
