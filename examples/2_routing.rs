//! # Routing Example
//!
//! Demonstrates how to define multiple routes and basic routing patterns.
//!
//! ## What You'll Learn
//! - Multiple route definitions
//! - Route matching and dispatch
//! - Different handler functions
//! - Simple static routes
//! - Timeout configuration
//!
//! ## Try It
//! ```bash
//! cargo run --example routing
//! curl http://127.0.0.1:8080/
//! curl http://127.0.0.1:8080/about
//! curl http://127.0.0.1:8080/api/status
//! curl http://127.0.0.1:8080/api/version
//! ```

use lumine::{IntoResponse, Lumine, Request, Result, http::StatusCode};
use std::{net::TcpListener, time::Duration};

fn main() -> Result<()> {
    let app = Lumine::builder()
        // Timeouts
        .read_timeout(Duration::from_secs(5))
        .write_timeout(Duration::from_secs(5))
        // Homepage
        .route("/", index_handler)
        // Static pages
        .route("/about", about_handler)
        .route("/health", health_handler)
        // API routes
        .route("/api/status", api_status_handler)
        .route("/api/version", api_version_handler)
        .build();

    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("✅ Server running at http://127.0.0.1:8080");
    println!("\n📍 Available Routes:");
    println!("  GET /               → Home page");
    println!("  GET /about          → About page");
    println!("  GET /health         → Health check");
    println!("  GET /api/status     → API status");
    println!("  GET /api/version    → API version");
    println!("\n💡 Try: curl http://127.0.0.1:8080/api/status");
    println!("⏹️  Press Ctrl+C to stop\n");

    app.serve(listener)
}

/// Handler: Homepage
fn index_handler(_req: Request) -> impl IntoResponse {
    "Welcome to Lumine! 🔥\n\nThis is the homepage."
}

/// Handler: About page
fn about_handler(_req: Request) -> impl IntoResponse {
    "About Lumine:\n\nLumine is a simple, synchronous HTTP web server written in Rust.\n\
     Design: Fast, Easy, No async/await complexity"
}

/// Handler: Health check (typically used by load balancers)
fn health_handler(_req: Request) -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

/// Handler: API status endpoint
fn api_status_handler(_req: Request) -> impl IntoResponse {
    (
        StatusCode::OK,
        "{\n  \"status\": \"online\",\n  \"version\": \"0.1.0\",\n  \"uptime\": \"100%\"\n}",
    )
}

/// Handler: API version endpoint
fn api_version_handler(_req: Request) -> impl IntoResponse {
    (
        StatusCode::OK,
        "{\n  \"api_version\": \"1.0.0\",\n  \"release\": \"2024-01-01\"\n}",
    )
}
