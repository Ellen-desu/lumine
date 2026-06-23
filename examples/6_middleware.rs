//! # Middleware Example
//!
//! Demonstrates how to use global and route-specific middleware in Lumine.
//!
//! ## What You'll Learn
//! - How to define a custom middleware
//! - Difference between global and route middleware
//! - Middleware execution order
//! - Using `route_with` for per-route configuration
//!
//! ## Try It
//! `bash
//! cargo run --example middleware
//! curl http://127.0.0.1:8080
//! `

use lumine::prelude::*;
use tokio::net::TcpListener;

/// A simple logging middleware (global)
struct Logger;

#[async_trait::async_trait]
impl Middleware for Logger {
    async fn handle(&self, request: Request, next: Next) -> Response {
        println!(
            "🌍 [Logger] Incoming request: {} {}",
            request.method(),
            request.uri()
        );

        let response = next.run(request).await;

        println!("🌍 [Logger] Response sent");

        response
    }
}

/// A route-specific authentication middleware
struct Auth;

#[async_trait::async_trait]
impl Middleware for Auth {
    async fn handle(&self, request: Request, next: Next) -> Response {
        println!("🔐 [Auth] Checking access...");

        let response = next.run(request).await;

        println!("🔐 [Auth] Finished processing");

        response
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Create application with:
    // - One route using route-specific middleware (Auth)
    // - One global middleware (Logger)
    let app = Lumine::builder()
        .route_with("/", hello_handler, |r| {
            // Attach Auth middleware ONLY to this route
            // and run it BEFORE global middleware
            r.middleware(Auth).run_before_global()
        })
        // Global middleware (applies to all routes)
        .middleware(Logger)
        .build();

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("✅ Server running at http://127.0.0.1:8080");
    println!("💡 Try: curl http://127.0.0.1:8080");
    println!("⏹️  Press Ctrl+C to stop\n");

    app.serve(listener).await
}

/// Simple handler
async fn hello_handler(_req: Request) -> impl IntoResponse {
    "Hello with Middleware!"
}
