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

use lumine::{IntoResponse, Lumine, Middleware, Next, Request, Response, Result};
use std::net::TcpListener;

/// A simple logging middleware (global)
struct Logger;

impl Middleware for Logger {
    fn handle(&self, request: Request, next: Next) -> Result<Response> {
        println!(
            "🌍 [Logger] Incoming request: {} {}",
            request.method(),
            request.uri()
        );

        let response = next.run(request);

        println!("🌍 [Logger] Response sent");

        response
    }
}

/// A route-specific authentication middleware
struct Auth;

impl Middleware for Auth {
    fn handle(&self, request: Request, next: Next) -> Result<Response> {
        println!("🔐 [Auth] Checking access...");

        let response = next.run(request);

        println!("🔐 [Auth] Finished processing");

        response
    }
}

fn main() -> Result<()> {
    // Create application with:
    // - One route using route-specific middleware (Auth)
    // - One global middleware (Logger)
    let app = Lumine::builder()
        .route_with("/", hello_handler, |r| {
            // Attach Auth middleware ONLY to this route
            // and run it BEFORE global middleware
            r.middleware(Auth).route_middleware_first()
        })
        // Global middleware (applies to all routes)
        .middleware(Logger)
        .build();

    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("✅ Server running at http://127.0.0.1:8080");
    println!("💡 Try: curl http://127.0.0.1:8080");
    println!("⏹️  Press Ctrl+C to stop\n");

    let rx = app.serve(listener);

    while let Ok(client) = rx.recv() {
        println!("[{}] {} {}", client.status(), client.method(), client.url());
    }

    Ok(())
}

/// Simple handler
fn hello_handler(_req: Request) -> impl IntoResponse {
    "Hello with Middleware!"
}
