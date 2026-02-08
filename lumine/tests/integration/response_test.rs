use lumine::{
    Lumine,
    http::{
        HeaderMap, HeaderValue, StatusCode,
        header::{CONNECTION, CONTENT_LENGTH, CONTENT_TYPE, DATE},
    },
};
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use std::net::TcpListener;

// ============================================================================
// Test Data Structures
// ============================================================================

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// ============================================================================
// 1. PLAIN TEXT RESPONSE TESTS
// ============================================================================

#[test]
fn test_plain_text_response() {
    let app = Lumine::builder()
        .route("/", |_| "Hello, World!")
        .route("/message", |_| "This is a plain text message")
        .build();

    let listener = TcpListener::bind("127.0.0.1:9001").unwrap();
    let _rx = app.serve(listener).unwrap();

    // Test root path
    let response = ureq::get("http://127.0.0.1:9001/").call().unwrap();
    let body = response.into_body().read_to_string().unwrap();
    assert_eq!(body, "Hello, World!");

    // Test message path
    let response = ureq::get("http://127.0.0.1:9001/message").call().unwrap();
    let body = response.into_body().read_to_string().unwrap();
    assert_eq!(body, "This is a plain text message");
}

#[test]
fn test_empty_response_body() {
    let app = Lumine::builder()
        .route("/empty", |_| "")
        .route("/void", |_| ())
        .build();

    let listener = TcpListener::bind("127.0.0.1:9002").unwrap();
    let _rx = app.serve(listener).unwrap();

    // Empty string
    let response = ureq::get("http://127.0.0.1:9002/empty").call().unwrap();
    let body = response.into_body().read_to_string().unwrap();
    assert_eq!(body, "");

    // Void response
    let response = ureq::get("http://127.0.0.1:9002/void").call().unwrap();
    let body = response.into_body().read_to_string().unwrap();
    assert_eq!(body, "");
}

// ============================================================================
// 2. JSON RESPONSE TESTS
// ============================================================================

#[test]
fn test_json_response_created() {
    let app = Lumine::builder()
        .route("/users", |_| {
            let user = User {
                id: 1,
                name: "John Doe".to_string(),
                email: "john@example.com".to_string(),
            };
            (StatusCode::CREATED, serde_json::to_string(&user).unwrap())
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:9003").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::post("http://127.0.0.1:9003/users")
        .send_empty()
        .unwrap();
    assert_eq!(response.status(), 201);

    let user: User = serde_json::from_reader(response.into_body().into_reader()).unwrap();
    assert_eq!(user.id, 1);
    assert_eq!(user.name, "John Doe");
}

#[test]
fn test_json_response_with_headers() {
    let app = Lumine::builder()
        .route("/api/data", |_| {
            let mut headers = HeaderMap::new();
            headers.append(CONTENT_TYPE, HeaderValue::from_static("application/json"));

            let data = serde_json::json!({
                "status": "success",
                "data": {
                    "id": 42,
                    "name": "Test"
                }
            });

            (
                StatusCode::OK,
                headers,
                serde_json::to_string(&data).unwrap(),
            )
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:9004").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:9004/api/data").call().unwrap();
    assert_eq!(response.status(), 200);
    assert_eq!(
        response.headers().get(CONTENT_TYPE).unwrap(),
        "application/json"
    );

    let body: serde_json::Value =
        serde_json::from_reader(response.into_body().into_reader()).unwrap();
    assert_eq!(body["status"], "success");
    assert_eq!(body["data"]["id"], 42);
}

#[test]
fn test_multiple_json_users() {
    let app = Lumine::builder()
        .route("/users/list", |_| {
            let users = vec![
                User {
                    id: 1,
                    name: "Alice".to_string(),
                    email: "alice@example.com".to_string(),
                },
                User {
                    id: 2,
                    name: "Bob".to_string(),
                    email: "bob@example.com".to_string(),
                },
            ];
            serde_json::to_string(&users).unwrap()
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:9005").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:9005/users/list")
        .call()
        .unwrap();
    let users: Vec<User> = serde_json::from_reader(response.into_body().into_reader()).unwrap();

    assert_eq!(users.len(), 2);
    assert_eq!(users[0].name, "Alice");
    assert_eq!(users[1].name, "Bob");
}

// ============================================================================
// 3. HTML RESPONSE TESTS
// ============================================================================

#[test]
fn test_html_response() {
    let app = Lumine::builder()
        .route("/page", |_| {
            let mut headers = HeaderMap::new();
            headers.append(CONTENT_TYPE, HeaderValue::from_static("text/html"));

            let html = r#"
                <!DOCTYPE html>
                <html>
                <head><title>Test Page</title></head>
                <body><h1>Hello</h1></body>
                </html>
            "#;

            (headers, html)
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:9006").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:9006/page").call().unwrap();
    assert_eq!(response.headers().get(CONTENT_TYPE).unwrap(), "text/html");

    let body = response.into_body().read_to_string().unwrap();
    assert_eq!(
        body.lines().map(str::trim).collect::<String>(),
        "<!DOCTYPE html><html><head><title>Test Page</title></head><body><h1>Hello</h1></body></html>"
    );
}

// ============================================================================
// 4. STATUS CODE TESTS
// ============================================================================

#[test]
fn test_status_codes() {
    let app = Lumine::builder()
        .route("/ok", |_| StatusCode::OK)
        .route("/created", |_| StatusCode::CREATED)
        .route("/accepted", |_| StatusCode::ACCEPTED)
        .route("/no-content", |_| StatusCode::NO_CONTENT)
        .route("/bad-request", |_| StatusCode::BAD_REQUEST)
        .route("/unauthorized", |_| StatusCode::UNAUTHORIZED)
        .route("/forbidden", |_| StatusCode::FORBIDDEN)
        .route("/not-found", |_| StatusCode::NOT_FOUND)
        .route("/conflict", |_| StatusCode::CONFLICT)
        .route("/server-error", |_| StatusCode::INTERNAL_SERVER_ERROR)
        .route("/not-implemented", |_| StatusCode::NOT_IMPLEMENTED)
        .build();

    let listener = TcpListener::bind("127.0.0.1:9007").unwrap();
    let _rx = app.serve(listener).unwrap();

    // 2xx Success
    assert_eq!(
        ureq::get("http://127.0.0.1:9007/ok")
            .call()
            .unwrap()
            .status(),
        200
    );
    assert_eq!(
        ureq::get("http://127.0.0.1:9007/created")
            .call()
            .unwrap()
            .status(),
        201
    );
    assert_eq!(
        ureq::get("http://127.0.0.1:9007/accepted")
            .call()
            .unwrap()
            .status(),
        202
    );
    assert_eq!(
        ureq::get("http://127.0.0.1:9007/no-content")
            .call()
            .unwrap()
            .status(),
        204
    );

    // 4xx Client Errors
    assert!(matches!(
        ureq::get("http://127.0.0.1:9007/bad-request").call(),
        Err(ureq::Error::StatusCode(400))
    ));
    assert!(matches!(
        ureq::get("http://127.0.0.1:9007/unauthorized").call(),
        Err(ureq::Error::StatusCode(401))
    ));
    assert!(matches!(
        ureq::get("http://127.0.0.1:9007/forbidden").call(),
        Err(ureq::Error::StatusCode(403))
    ));

    // 5xx Server Errors
    assert!(matches!(
        ureq::get("http://127.0.0.1:9007/server-error").call(),
        Err(ureq::Error::StatusCode(500))
    ));
    assert!(matches!(
        ureq::get("http://127.0.0.1:9007/server-error").call(),
        Err(ureq::Error::StatusCode(500))
    ));
    assert!(matches!(
        ureq::get("http://127.0.0.1:9007/not-implemented").call(),
        Err(ureq::Error::StatusCode(501))
    ));
}

#[test]
fn test_u16_status_code() {
    let app = Lumine::builder()
        .route("/code-200", |_| 200u16)
        .route("/code-201", |_| 201u16)
        .route("/code-404", |_| 404u16)
        .route("/code-500", |_| 500u16)
        .build();

    let listener = TcpListener::bind("127.0.0.1:9008").unwrap();
    let _rx = app.serve(listener).unwrap();

    assert_eq!(
        ureq::get("http://127.0.0.1:9008/code-200")
            .call()
            .unwrap()
            .status(),
        200
    );
    assert_eq!(
        ureq::get("http://127.0.0.1:9008/code-201")
            .call()
            .unwrap()
            .status(),
        201
    );

    assert!(matches!(
        ureq::get("http://127.0.0.1:9008/code-404").call(),
        Err(ureq::Error::StatusCode(404))
    ));
    assert!(matches!(
        ureq::get("http://127.0.0.1:9008/code-500").call(),
        Err(ureq::Error::StatusCode(500))
    ));
}

#[test]
fn test_invalid_u16_status_code() {
    let app = Lumine::builder()
        .route("/invalid", |_| 999u16) // Invalid status code
        .build();

    let listener = TcpListener::bind("127.0.0.1:9009").unwrap();
    let _rx = app.serve(listener).unwrap();

    // Should return 500 when invalid status code is provided
    assert!(matches!(
        ureq::get("http://127.0.0.1:9009/invalid").call(),
        Err(ureq::Error::StatusCode(500))
    ));
}

// ============================================================================
// 5. RESPONSE WITH CUSTOM HEADERS TESTS
// ============================================================================

#[test]
fn test_custom_headers_only() {
    let app = Lumine::builder()
        .route("/with-headers", |_| {
            let mut headers = HeaderMap::new();
            headers.append("X-Custom-Header", HeaderValue::from_static("custom-value"));
            headers.append("X-Request-ID", HeaderValue::from_static("123-abc"));

            (headers, "Body content")
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:9010").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:9010/with-headers")
        .call()
        .unwrap();

    let headers = response.headers();
    assert_eq!(headers.get("x-custom-header").unwrap(), "custom-value");
    assert_eq!(headers.get("x-request-id").unwrap(), "123-abc");
}

#[test]
fn test_status_code_with_headers_and_body() {
    let app = Lumine::builder()
        .route("/full-response", |_| {
            let mut headers = HeaderMap::new();
            headers.append(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
            headers.append("X-Response-Time", HeaderValue::from_static("100ms"));

            (StatusCode::ACCEPTED, headers, "Response accepted")
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:9011").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:9011/full-response")
        .call()
        .unwrap();
    assert_eq!(response.status(), 202);

    let headers = response.headers();
    assert_eq!(headers.get(CONTENT_TYPE).unwrap(), "text/plain");
    assert_eq!(headers.get("x-response-time").unwrap(), "100ms");

    let body = response.into_body().read_to_string().unwrap();
    assert_eq!(body, "Response accepted");
}

// ============================================================================
// 6. DEFAULT HEADERS TESTS
// ============================================================================

#[test]
fn test_default_headers_are_set() {
    let app = Lumine::builder()
        .route("/check-headers", |_| "test content")
        .build();

    let listener = TcpListener::bind("127.0.0.1:9012").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:9012/check-headers")
        .call()
        .unwrap();

    let headers = response.headers();

    // Should have default headers set by framework
    assert!(headers.contains_key(CONTENT_LENGTH));
    assert!(headers.contains_key(DATE));
    assert!(headers.contains_key(CONNECTION));

    // Content-Type should be set to text/plain if not explicitly set
    assert_eq!(headers.get(CONTENT_TYPE).unwrap(), "text/plain");
}

#[test]
fn test_content_length_header() {
    let app = Lumine::builder()
        .route("/short", |_| "hi")
        .route(
            "/long",
            |_| "This is a much longer content that should have a bigger content length",
        )
        .build();

    let listener = TcpListener::bind("127.0.0.1:9013").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response1 = ureq::get("http://127.0.0.1:9013/short").call().unwrap();
    let len1 = response1
        .headers()
        .get(CONTENT_LENGTH)
        .unwrap()
        .to_str()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    assert_eq!(len1, 2);

    let response2 = ureq::get("http://127.0.0.1:9013/long").call().unwrap();
    let len2 = response2
        .headers()
        .get(CONTENT_LENGTH)
        .unwrap()
        .to_str()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    assert!(len2 > len1);
}

#[test]
fn test_no_content_type_for_empty_body() {
    let app = Lumine::builder().route("/empty", |_| ()).build();

    let listener = TcpListener::bind("127.0.0.1:9014").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:9014/empty").call().unwrap();
    // Empty response should not have content-type header
    assert!(response.headers().get(CONTENT_TYPE).is_none());
}

// ============================================================================
// 7. RESPONSE SIZE TESTS
// ============================================================================

#[test]
fn test_large_response_body() {
    let large_content = "x".repeat(5000);
    let content_clone = large_content.clone();

    let app = Lumine::builder()
        .route("/large", move |_| content_clone.clone())
        .build();

    let listener = TcpListener::bind("127.0.0.1:9015").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:9015/large").call().unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert_eq!(body.len(), 5000);
    assert_eq!(body, large_content);
}

#[test]
fn test_binary_response() {
    let app = Lumine::builder()
        .route("/binary", |_| vec![0u8, 1, 2, 3, 255, 254])
        .build();

    let listener = TcpListener::bind("127.0.0.1:9016").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:9016/binary").call().unwrap();
    let body = response.into_body().read_to_vec().unwrap();

    assert_eq!(body, vec![0u8, 1, 2, 3, 255, 254]);
}

// ============================================================================
// 8. 404 AND ERROR RESPONSE TESTS
// ============================================================================

#[test]
fn test_404_not_found() {
    let app = Lumine::builder().route("/exists", |_| "found").build();

    let listener = TcpListener::bind("127.0.0.1:9017").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:9017/exists").call().unwrap();
    assert_eq!(response.status(), 200);

    let response = ureq::get("http://127.0.0.1:9017/not-exists").call();
    assert!(matches!(response, Err(ureq::Error::StatusCode(404))));
}

#[test]
fn test_413_payload_too_large() {
    let app = Lumine::builder().route("/upload", |_| "ok").build();

    let listener = TcpListener::bind("127.0.0.1:9018").unwrap();
    let _rx = app.serve(listener).unwrap();

    // Default max body is 1024 bytes
    let large_body = vec![0u8; 2048];
    let response = ureq::post("http://127.0.0.1:9018/upload").send(&large_body);

    assert!(matches!(response, Err(ureq::Error::StatusCode(413))));
}

// ============================================================================
// 9. RESPONSE WITH VEC<U8> TESTS
// ============================================================================

#[test]
fn test_vec_u8_response() {
    let app = Lumine::builder()
        .route("/bytes", |_| {
            vec![72, 101, 108, 108, 111] // "Hello" in ASCII
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:9019").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:9019/bytes").call().unwrap();
    let body = response.into_body().read_to_vec().unwrap();

    assert_eq!(body, vec![72, 101, 108, 108, 111]);
}

// ============================================================================
// 10. CONTENT-TYPE AUTO-DETECTION TESTS
// ============================================================================

#[test]
fn test_default_content_type_text_plain() {
    let app = Lumine::builder()
        .route("/text", |_| "plain text response")
        .build();

    let listener = TcpListener::bind("127.0.0.1:9020").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:9020/text").call().unwrap();
    assert_eq!(response.headers().get(CONTENT_TYPE).unwrap(), "text/plain");
}

#[test]
fn test_custom_content_type_override() {
    let app = Lumine::builder()
        .route("/custom", |_| {
            let mut headers = HeaderMap::new();
            headers.append(CONTENT_TYPE, HeaderValue::from_static("application/custom"));

            (headers, "custom content")
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:9021").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:9021/custom").call().unwrap();
    assert_eq!(
        response.headers().get(CONTENT_TYPE).unwrap(),
        "application/custom"
    );
}
