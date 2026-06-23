//! # Response Types Example
//!
//! Demonstrates different response types and content types.
//!
//! ## What You'll Learn
//! - Plain text responses
//! - HTML responses with proper Content-Type
//! - JSON responses with proper Content-Type
//! - Status codes and HTTP semantics
//! - File responses with FileStream (requires `filestream` feature)
//! - Custom response headers
//! - Response tuples: (StatusCode, Body), (Headers, Body), etc.
//!
//! ## Try It
//! ```bash
//! cargo run --example response
//! curl http://127.0.0.1:8080/text
//! curl http://127.0.0.1:8080/html
//! curl http://127.0.0.1:8080/json
//! curl http://127.0.0.1:8080/inline (needs to open in a browser)
//! curl http://127.0.0.1:8080/attachment --output lumine.png
//! curl http://127.0.0.1:8080/created
//! curl http://127.0.0.1:8080/empty
//! ```

use lumine::prelude::*;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Lumine::builder()
        // Plain text response
        .route("/text", text_response)
        // HTML response
        .route("/html", html_response)
        // JSON response
        .route("/json", json_response)
        // File response
        .route("/inline", inline_response)
        .route("/attachment", attachment_response)
        // Response with status code
        .route("/created", created_response)
        // Response with headers
        .route("/custom-headers", custom_headers_response)
        // Empty response
        .route("/empty", empty_response)
        .build();

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("✅ Server running at http://127.0.0.1:8080");
    println!("\n📍 Response Examples:");
    println!("  GET /text              → Plain text (auto text/plain)");
    println!("  GET /html              → HTML page (text/html)");
    println!("  GET /json              → JSON data (application/json)");
    println!("  GET /inline            → File content (auto Content-Type)");
    println!("  GET /attachment        → File content (auto Content-Type)");
    println!("  GET /created           → Status 201 Created");
    println!("  GET /custom-headers    → Custom response headers");
    println!("  GET /empty             → Empty body");
    println!("\n💡 Try: curl -i http://127.0.0.1:8080/json");
    println!("⏹️  Press Ctrl+C to stop\n");

    app.serve(listener).await
}

/// Handler: Plain text response
///
/// Lumine automatically sets Content-Type: text/plain for string responses
async fn text_response(_req: Request) -> impl IntoResponse {
    "This is plain text.\nMultiple lines are supported.\nAs well as special chars: !@#$%^&*()"
}

/// Handler: HTML response with explicit Content-Type header
async fn html_response(_req: Request) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.append(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/html; charset=utf-8"),
    );

    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Lumine HTML Example</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            max-width: 800px;
            margin: 50px auto;
            padding: 20px;
            background: #f5f5f5;
        }
        h1 { color: #333; }
        p { color: #666; }
    </style>
</head>
<body>
    <h1>🔥 Welcome to Lumine</h1>
    <p>This is an HTML response served by Lumine HTTP server.</p>
    <p><strong>Features:</strong></p>
    <ul>
        <li>Synchronous request handling</li>
        <li>Simple routing</li>
        <li>Multiple response types</li>
    </ul>
</body>
</html>"#;

    (headers, html)
}

/// Handler: JSON response with explicit Content-Type header
///
/// Note: JSON is passed as a string, not structured data
/// Use serde_json for complex JSON generation
async fn json_response(_req: Request) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.append(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );

    // In real applications, use serde_json::to_string() for complex data
    let json = r#"{
  "status": "success",
  "message": "This is a JSON response",
  "data": {
    "version": "0.1.0",
    "features": [
      "Routing",
      "Multiple response types",
      "Request parameters"
    ]
  }
}"#;

    (StatusCode::OK, headers, json)
}

/// Handler: Inline response with Content-Disposition: inline
async fn inline_response(_req: Request) -> impl IntoResponse {
    match FileStream::open_with_disposition("assets/lumine.png", Disposition::Inline).await {
        Ok(stream) => stream,
        Err(_) => panic!(""),
    }
}

/// Handler: Attachment response with Content-Disposition: attachment
async fn attachment_response(_req: Request) -> impl IntoResponse {
    match FileStream::open_with_disposition("assets/lumine.png", Disposition::Attachment).await {
        Ok(stream) => stream,
        Err(_) => panic!(""),
    }
}

/// Handler: Response with 201 Created status code
///
/// Useful for POST/CREATE endpoints
async fn created_response(_req: Request) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.append(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );

    let response = r#"{
  "status": "created",
  "id": 42,
  "message": "Resource successfully created"
}"#;

    (StatusCode::CREATED, headers, response)
}

/// Handler: Response with custom headers
async fn custom_headers_response(_req: Request) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.append(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    headers.append("X-Custom-Header", HeaderValue::from_static("custom-value"));
    headers.append("X-Request-ID", HeaderValue::from_static("req-12345"));
    headers.append(
        "Cache-Control",
        HeaderValue::from_static("no-cache, no-store"),
    );

    let json = r#"{
  "message": "Response with custom headers",
  "headers": {
    "X-Custom-Header": "custom-value",
    "X-Request-ID": "req-12345",
    "Cache-Control": "no-cache, no-store"
  }
}"#;

    (headers, json)
}

/// Handler: Empty response body
async fn empty_response(_req: Request) -> impl IntoResponse {
    (StatusCode::NO_CONTENT, "")
}
