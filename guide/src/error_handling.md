# Error Handling

Robust error handling is a key requirement for high-quality web servers. Lumine leverages Rust's safe error handling system and provides deep integration with HTTP status codes.

## Returning Result from Handlers

Lumine implements the `IntoResponse` trait for `std::result::Result<T, T>` where `T` implements `IntoResponse`. This allows you to use the `?` operator to streamline your handler's asynchronous logic:

```rust,no_run
# use lumine::prelude::*;
# 
# #[tokio::main]
# async fn main() {
#     let app = Lumine::builder()
#         .route("/users", db_fetch_handler)
#         .build();
#
#     // ... run server
# }
#
async fn db_fetch_handler(req: Request) -> Result<String, StatusCode> {
    let user_id = extract_user_id(&req)?; // Returns Err(StatusCode::BAD_REQUEST) on failure

    match fetch_user_from_db(user_id).await {
        Ok(user) => Ok(format!("Hello, {}", user.name)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
```

---

## Mapping System Errors to HTTP Status Codes

Lumine has an internal `Error` enum that automatically maps protocol and parsing issues to appropriate HTTP status codes:

- `Error::UriTooLarge` -> `414 URI TOO LONG`
- `Error::BodyTooLarge` -> `413 PAYLOAD TOO LARGE`
- `Error::HeadersTooLarge` -> `431 REQUEST HEADER FIELDS TOO LARGE`
- `Error::QueryTooLarge` -> `414 URI TOO LONG`
- `Error::HttpVersionNotSupported` -> `505 HTTP VERSION NOT SUPPORTED`
- `Error::InvalidRequestLine` / `Error::InvalidHeaders` -> `400 BAD REQUEST`
- `Error::Unimplemented` -> `501 NOT IMPLEMENTED`

If any of these errors occur while receiving a request, Lumine intercepts the request and gracefully returns the corresponding HTTP response without crashing the server.

---

## Creating Application-Specific Custom Errors

For larger applications, it is highly recommended to create a custom error enum and implement `IntoResponse` to map internal errors to user-friendly HTTP responses:

```rust,no_run
# use lumine::prelude::*;
# 
# #[tokio::main]
# async fn main() {
#     let app = Lumine::builder()
#         .route("/users", get_user_profile)
#         .build();
#
#     // ... run server
# }
#
use serde::Serialize;

#[derive(Debug)]
enum AppError {
    NotFound(String),
    DatabaseError,
    ValidationError(String),
}

#[derive(Serialize)]
struct ErrorPayload {
    error: String,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(res) => (StatusCode::NOT_FOUND, format!("Resource '{}' not found", res)),
            AppError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal database error occurred".to_string()),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let payload = ErrorPayload {
            error: format!("{:?}", self),
            message,
        };

        let json_body = serde_json::to_string(&payload).unwrap();

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        (status, headers, json_body).into_response()
    }
}

// Your handler remains extremely clean and safe:
async fn get_user_profile(req: Request) -> Result<String, AppError> {
    let id = extract_id(&req).map_err(|_| AppError::ValidationError("Invalid ID".to_string()))?;
    
    let user = db_query_user(id).await
        .map_err(|_| AppError::DatabaseError)?
        .ok_or_else(|| AppError::NotFound("User".to_string()))?;

    Ok(serde_json::to_string(&user).unwrap())
}
```
