# Handling Requests

Each route handler receives a `Request` parameter, which provides full access to all HTTP data sent by the client.

## Reading Method, URI, and Headers

You can easily access basic details of the `Request` object:

```rust,no_run
# use lumine::prelude::*;
# 
# #[tokio::main]
# async fn main() {
#     let app = Lumine::builder()
#         .route("/", request_info_handler)
#         .build();
#
#     // ... run server
# }
#
async fn request_info_handler(req: Request) -> impl IntoResponse {
    // Read the HTTP Method (GET, POST, etc.)
    let method = req.method().to_string();

    // Read the Request URI
    let uri = req.uri().to_string();

    // Read a specific Header
    let user_agent = req
        .headers()
        .get("user-agent")
        .and_then(|val| val.to_str().ok())
        .unwrap_or("Unknown");

    format!(
        "Method: {}\nURI: {}\nUser-Agent: {}",
        method, uri, user_agent
    )
}
```

---

## Reading the Request Body

Lumine stores the request body bytes directly in the `Request` struct. You can access these bytes via `req.body()`.

### Parsing JSON from the Body
To read JSON sent by the client, you can combine Lumine with the `serde_json` crate to perform safe deserialization:

```rust,no_run
# use lumine::prelude::*;
# 
# #[tokio::main]
# async fn main() {
#     let app = Lumine::builder()
#         .route("/users", create_user_handler)
#         .build();
#
#     // ... run server
# }
#
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateUserDto {
    username: String,
    email: String,
}

async fn create_user_handler(req: Request) -> impl IntoResponse {
    // Ensure the HTTP Method is POST
    if req.method() != "POST" {
        return (StatusCode::METHOD_NOT_ALLOWED, "Method must be POST").into_response();
    }

    // Parse the body bytes into the struct
    let payload: CreateUserDto = match serde_json::from_slice(req.body()) {
        Ok(data) => data,
        Err(err) => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Invalid JSON: {}", err)
            ).into_response();
        }
    };

    format!("User '{}' successfully created!", payload.username).into_response()
}
```
