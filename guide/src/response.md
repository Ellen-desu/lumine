# Generating Responses

Lumine uses the `IntoResponse` trait to automatically convert values returned by handlers into valid HTTP responses. This gives developers the flexibility to return the data type that best fits their needs.

## Built-in Types Supporting `IntoResponse`

By default, Lumine provides `IntoResponse` implementations for many common types:

### 1. Plain Text (`&'static str` and `String`)
Automatically generates a `200 OK` response with a `Content-Type: text/plain` header.
```rust,no_run
# use lumine::prelude::*;
# 
# #[tokio::main]
# async fn main() {
#     let app = Lumine::builder()
#         .route("/plain", handler)
#         .build();
#
#     // ... run server
# }
#
async fn handler(_req: Request) -> impl IntoResponse {
    "This is plain text"
}
```

### 2. Status Code (`StatusCode`)
Generates an empty response with the specified HTTP status.
```rust,no_run
# use lumine::prelude::*;
# 
# #[tokio::main]
# async fn main() {
#     let app = Lumine::builder()
#         .route("/not-found", not_found_handler)
#         .build();
#
#     // ... run server
# }
#
async fn not_found_handler(_req: Request) -> impl IntoResponse {
    StatusCode::NOT_FOUND
}
```

---

## Using Tuples for Flexible Responses

If you need to customize the HTTP status or add additional headers, Lumine supports returning tuples:

### `(StatusCode, Body)`
Specify the status code and body together.
```rust,no_run
# use lumine::prelude::*;
# 
# #[tokio::main]
# async fn main() {
#     let app = Lumine::builder()
#         .route("/created", created_handler)
#         .build();
#
#     // ... run server
# }
#
async fn created_handler(_req: Request) -> impl IntoResponse {
    (StatusCode::CREATED, "Resource successfully created")
}
```

### `(HeaderMap, Body)`
Specify custom headers and a body. The status defaults to `200 OK`.
```rust,no_run
# use lumine::prelude::*;
# 
# #[tokio::main]
# async fn main() {
#     let app = Lumine::builder()
#         .route("/custom-header", custom_header_handler)
#         .build();
#
#     // ... run server
# }
#
async fn custom_header_handler(_req: Request) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("X-App-Version", "1.0.0".parse().unwrap());

    (headers, "Text with custom header")
}
```

### `(StatusCode, HeaderMap, Body)`
Customize the status code, headers, and body all at once. This gives you full control over your response.
```rust,no_run
# use lumine::prelude::*;
# 
# #[tokio::main]
# async fn main() {
#     let app = Lumine::builder()
#         .route("/full-response", full_response_handler)
#         .build();
#
#     // ... run server
# }
#
async fn full_response_handler(_req: Request) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let json_data = r#"{"status": "ok"}"#;

    (StatusCode::OK, headers, json_data)
}
```

---

## Streaming Files

If you enable the `filestream` feature, Lumine provides asynchronous handling to stream files directly from your filesystem efficiently:

```rust,no_run
# use lumine::prelude::*;
#
# #[tokio::main]
# async fn main() {
#     let app = Lumine::builder()
#         .route("/download", download_image_handler)
#         .build();
#
#     // ... run server
# }
#
use lumine::filestream::{FileStream, Disposition};

async fn download_image_handler(_req: Request) -> impl IntoResponse {
    // Open file in inline mode (displayed in browser) or attachment mode (downloaded)
    match FileStream::open_with_disposition("assets/logo.png", Disposition::Attachment).await {
        Ok(stream) => stream.into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "File not found").into_response(),
    }
}
```
