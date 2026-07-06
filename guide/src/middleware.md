# Middleware

Middleware in Lumine allows you to run code before a request reaches the main handler (pre-processing) and after the handler returns a response (post-processing). This is ideal for cross-cutting concerns like logging, authentication, CORS, compression, etc.

## Defining Custom Middleware

To create a new middleware, define a struct and implement the `Middleware` trait:

```rust,no_run
use lumine::prelude::*;
#
# #[tokio::main]
# async fn main() {}

struct RequestLogger;

#[async_trait::async_trait]
impl Middleware for RequestLogger {
    async fn handle(&self, request: Request, next: Next) -> Response {
        // 1. Pre-processing: Executed BEFORE the request reaches the handler
        println!("🌍 Incoming request: {} {}", request.method(), request.uri());

        // 2. Run the rest of the chain (other middleware or the handler)
        let response = next.run(request).await;

        // 3. Post-processing: Executed AFTER the handler finishes
        println!("🌍 Finished processing. Response status: {}", response.status());

        // Return the response object
        response
    }
}
```

---

## Registering Global Middleware

Global middleware is applied to all registered routes in the application. Register it using `.middleware(...)` on your application builder.

```rust,no_run
# use lumine::prelude::*;
#
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Lumine::builder()
        .route("/", home_handler)
        .route("/about", about_handler)
        // Register RequestLogger as global middleware
        .middleware(RequestLogger)
        .build();

    // ... run server
}
```

---

## Registering Route-Specific Middleware

If you only want to apply middleware to specific routes (e.g., an `/admin` route requiring authentication), you can use the `.route_with(...)` method:

```rust,no_run
# use lumine::prelude::*;
#
struct AuthMiddleware;

#[async_trait::async_trait]
impl Middleware for AuthMiddleware {
    async fn handle(&self, request: Request, next: Next) -> Response {
        // Token verification logic here
        let is_authenticated = check_token(&request);

        if !is_authenticated {
            // Intercept the request and immediately return 401 Unauthorized
            return (StatusCode::UNAUTHORIZED, "Access denied").into_response();
        }

        next.run(request).await
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Lumine::builder()
        // Public route
        .route("/", home_handler)
        // Protected route with specific middleware
        .route_with("/admin", admin_dashboard_handler, |r| {
            r.middleware(AuthMiddleware).run_before_global()
        })
        .middleware(RequestLogger) // Global logger still runs
        .build();
    
    // ... run server
}
```
