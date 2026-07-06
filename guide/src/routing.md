# Routing & Parameters

Lumine provides a highly flexible path-based dynamic routing system. You can register routes directly using `Lumine::builder().route(...)`.

## Registering Static Routes

Static routes are perfect for pages that don't require dynamic parameters in their URLs.

```rust,no_run
# use lumine::prelude::*;
#
# #[tokio::main]
# async fn main() {
    let app = Lumine::builder()
        .route("/", index_handler)
        .route("/about", about_handler)
        .route("/api/v1/status", status_handler)
        .build();
    
    // ... run server
# }
```

---

## Path Parameters

Lumine supports defining dynamic parameters in the path using the colon (`:id`) syntax.

### Route Registration Example
```rust,no_run
# use lumine::prelude::*;
#
# #[tokio::main]
# async fn main() {
    let app = Lumine::builder()
        .route("/users/:id", get_user_handler)
        .route("/users/:user_id/posts/:postId", get_post_handler)
        .build();
    
    // ... run server
# }
```

### Extracting Path Parameters
Inside a route handler, you can use the `Params::from_request` helper to extract these parameters:

```rust,no_run
# use lumine::prelude::*;
# 
# #[tokio::main]
# async fn main() {
#     let app = Lumine::builder()
#         .route("/users/:id", get_user_handler)
#         .build();
#
#     // ... run server
# }
#
async fn get_user_handler(req: Request) -> impl IntoResponse {
    // Extract path parameters from the request
    let params = Params::from_request(&req);

    // Retrieve the parameter by the name registered in the route
    let user_id = match params.get("id") {
        Some(id) => id,
        None => return (StatusCode::BAD_REQUEST, "Missing user id").into_response(),
    };

    format!("Fetching data for user with ID: {}", user_id).into_response()
}
```

---

## Query Parameters

Query parameters are optional parameters sent at the end of the URL (e.g., `?search=rust&limit=10`). You do not need to register them in the route definition; simply extract them within the handler.

### Extracting Query Parameters
Use the `Query::from_request` helper to retrieve query parameters:

```rust,no_run
# use lumine::prelude::*;
# 
# #[tokio::main]
# async fn main() {
#     let app = Lumine::builder()
#         .route("/search", search_handler)
#         .build();
#
#     // ... run server
# }
#
async fn search_handler(req: Request) -> impl IntoResponse {
    // Extract query parameters from the request
    let query_params = Query::from_request(&req);

    // Retrieve the "q" parameter (search query)
    let search_query = query_params.get("q").unwrap_or("default_search");

    // Retrieve the "limit" parameter, parsing it to a numeric type
    let limit = query_params
        .get("limit")
        .and_then(|l| l.parse::<usize>().ok())
        .unwrap_or(10); // Default value if limit is missing or invalid

    format!("Search results for '{}' with limit: {}", search_query, limit)
}
```
