//! # Static Files Example
//!
//! Demonstrates how to serve static files from a directory using `Remainder` and `FileStream`.
//!
//! ## What You'll Learn
//! - Using wildcard routes (`/*`)
//! - Extracting the `Remainder` of a path
//! - Serving files with `FileStream`
//! - Handling 404 Not Found for missing files
//!
//! ## Try It
//! ```bash
//! cargo run --example static_files
//! curl http://127.0.0.1:8080/assets/lumine.png --output test.png
//! curl http://127.0.0.1:8080/assets/missing.txt
//! ```

use lumine::prelude::*;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Lumine::builder()
        // Match any request starting with /assets/
        // The * will capture the remainder of the URI
        .route("/assets/*", serve_static_file)
        .build();

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("✅ Server running at http://127.0.0.1:8080");
    println!("💡 Try: curl -O http://127.0.0.1:8080/assets/lumine.png");
    println!("⏹️  Press Ctrl+C to stop\n");

    app.serve(listener).await;

    Ok(())
}

/// Handler: Serves files from the `assets/` directory
async fn serve_static_file(req: Request) -> impl IntoResponse {
    // Extract the wildcard part of the route
    let remainder = Remainder::from_request(&req);

    let path = match remainder.get() {
        Some(p) if !p.is_empty() => p,
        _ => return (StatusCode::BAD_REQUEST, "Invalid file path").into_response(),
    };

    // Construct the actual file path on the filesystem
    // WARNING: In a real app, ensure the path does not contain ".." to prevent directory traversal
    if path.contains("..") {
        return (StatusCode::BAD_REQUEST, "Directory traversal not allowed").into_response();
    }

    let file_path = format!("assets/{}", path);

    // Try to open the file
    match FileStream::open(&file_path).await {
        Ok(stream) => stream.into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "File not found").into_response(),
    }
}
