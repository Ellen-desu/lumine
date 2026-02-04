use std::net::TcpListener;

use http::StatusCode;
use lumine::{
    IntoResponse, Lumine, Request, Result,
    http::{HeaderMap, HeaderValue, header::CONTENT_TYPE},
};

fn main() -> Result<()> {
    let app = Lumine::builder()
        .route("/plain", plain_response)
        .route("/html", html_response)
        .route("/json", json_response)
        .build();

    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // You have to handle client error and do a while loop inside it. Otherwise, the event loop never starts
    if let Ok(rx) = app.serve(listener) {
        while let Ok(error) = rx.recv() {
            eprintln!("Client error: {error}");
        }
    }

    Ok(())
}

fn plain_response(_: Request) -> impl IntoResponse {
    // By default, response body will automatically use text/plain content-type header if doesn't set up
    "Hello, World!"
}

fn html_response(_: Request) -> impl IntoResponse {
    let mut header = HeaderMap::new();
    header.append(CONTENT_TYPE, HeaderValue::from_static("text/html"));

    let html = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Document Title</title>
        </head>
        <body>
            <h1>Hello, World!</h1>
        </body>
        </html>
        "#;

    (header, html)
}

fn json_response(_: Request) -> impl IntoResponse {
    let mut header = HeaderMap::new();
    header.append(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // You can use serde to parsing the json or anything
    let json = r#"
        {
            "name": "John Doe",
            "gay": false
        }
        "#;

    (StatusCode::OK, header, json)
}
