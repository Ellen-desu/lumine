//! # Hello World Example
//!
//! The simplest possible Lumine server.
//!
//! ## What You'll Learn
//! - Basic server setup with Lumine::builder()
//! - Single route definition
//! - Server binding and serving
//!
//! ## Try It
//! ```bash
//! cargo run --example hello_world
//! curl http://127.0.0.1:8080
//! ```

use lumine::prelude::*;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Create application with one simple route
    let app = Lumine::builder().route("/", hello_handler).build();

    // Bind to localhost:8080
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("✅ Server running at http://127.0.0.1:8080");
    println!("💡 Try: curl http://127.0.0.1:8080");
    println!("⏹️  Press Ctrl+C to stop\n");

    app.serve(listener).await
}
/// Simple handler that returns "Hello, World!"
async fn hello_handler(_req: Request) -> impl IntoResponse {
    "Hello, World!"
}
