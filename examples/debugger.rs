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

use lumine::{IntoResponse, Lumine, Request, Response, Result, attachment::Attachment};
use std::net::TcpListener;

struct Logger;

impl lumine::Middleware for Logger {
    fn handle(&self, req: Request, next: lumine::Next) -> Result<Response> {
        let response = next.run(req)?;

        Ok(response)
    }
}

fn main() -> Result<()> {
    // Create application with one simple route
    let app = Lumine::builder()
        .route("/", attach)
        .middleware(Logger)
        .build();

    // Bind to localhost:8080
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("✅ Server running at http://127.0.0.1:8080");
    println!("💡 Try: curl http://127.0.0.1:8080");
    println!("⏹️  Press Ctrl+C to stop\n");

    app.serve(listener)
}

/// Simple handler that returns "Hello, World!"
fn attach(_req: Request) -> impl IntoResponse {
    Attachment::open("/home/airi/Projects/lumine/LICENSE", "LICENSE")
}
