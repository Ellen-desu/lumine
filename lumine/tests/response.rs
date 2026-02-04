use std::net::TcpListener;

use lumine::{Lumine, Result, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct User<'a> {
    id: u32,
    name: &'a str,
}

#[test]
fn response() -> Result<()> {
    let app = Lumine::builder()
        .route("/", |_| {})
        .route("/json", |_| {
            (
                StatusCode::from_u16(201).unwrap(),
                serde_json::to_string(&User {
                    id: 1,
                    name: "John Doe",
                })
                .unwrap(),
            )
        })
        .route("/invalid-status-code", |_| 99)
        .build();
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    app.serve(listener)?;

    test_send_body();
    test_not_found();
    test_max_body();

    Ok(())
}

fn test_send_body() {
    let mut response = ureq::get("http://127.0.0.1:8080/json").call().unwrap();

    let body = response.body_mut().read_to_string().unwrap();
    let status = response.status().as_u16();

    assert_eq!(status, 201);
    assert_eq!(
        serde_json::from_str::<User>(&body).unwrap(),
        User {
            id: 1,
            name: "John Doe",
        }
    );
}

fn test_not_found() {
    let response_result = ureq::get("http://127.0.0.1:8080/nothing").call();
    assert!(
        matches!(response_result, Err(ureq::Error::StatusCode(404))),
        "API doesn't respond as expected"
    );
}

pub fn test_max_body() {
    // By default, lumine maximum body is 1KB or 1024 bytes
    let body = vec![0u8; 2048];
    let response_result = ureq::post("http://127.0.0.1:8080").send(&body);
    assert!(
        matches!(response_result, Err(ureq::Error::StatusCode(413))),
        "API doesn't respond as expected"
    );

    let body = vec![0u8; 1024];
    let response_result = ureq::post("http://127.0.0.1:8080").send(&body);
    assert!(response_result.is_ok())
}
