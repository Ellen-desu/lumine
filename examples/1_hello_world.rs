//! # Hello World Example
//!
//! The simplest possible Lumine server.
//!
//! ## What You'll Learn
//! - Basic server setup with Lumine::builder()
//! - Single route definition
//! - Server binding and serving
//! - Error handling with Result
//!
//! ## Try It
//! ```bash
//! cargo run --example hello_world
//! curl http://127.0.0.1:8080
//! ```

use lumine::{IntoResponse, Lumine, Request, Result};
use std::net::TcpListener;

fn main() -> Result<()> {
    // Create application with one simple route
    let app = Lumine::builder().route("/", hello_handler).build();

    // Bind to localhost:8080
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("✅ Server running at http://127.0.0.1:8080");
    println!("💡 Try: curl http://127.0.0.1:8080");
    println!("⏹️  Press Ctrl+C to stop\n");

    // Start serving requests
    let rx = app.serve(listener);

    // ⚠️ IMPORTANT: This loop is required to keep server running!
    while let Ok(client) = rx.recv() {
        // Client object contains request metadata
        println!("[{}] {} {}", client.status(), client.method(), client.url());
    }

    Ok(())
}

/// Simple handler that returns "Hello, World!"
fn hello_handler(_req: Request) -> impl IntoResponse {
    "Hello, World!"
}
