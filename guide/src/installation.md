# Installation & Quick Start

To start using Lumine, ensure you have the Rust compiler and cargo installed on your system.

## Adding Dependencies

Add `lumine` and other supporting dependencies to your project's `Cargo.toml`:

```toml
[dependencies]
lumine = { version = "0" }
tokio = { version = "1", features = ["full"] }
anyhow = "1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

> **Note**: Within this workspace, you can refer to the local `lumine` folder.

---

## Your First "Hello World" Application

Create a new file at `src/main.rs` and write the following code:

```rust,no_run
use lumine::prelude::*;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Build the application with a "/" route
    let app = Lumine::builder()
        .route("/", hello_handler)
        .build();

    // 2. Bind to localhost:8080
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("✅ Server running at http://127.0.0.1:8080");

    // 3. Serve the application
    app.serve(listener).await;

    Ok(())
}

// Simple route handler that returns "Hello, World!"
async fn hello_handler(_req: Request) -> impl IntoResponse {
    "Hello, World!"
}
```

## Running the Server

Run the server using cargo:

```bash
cargo run
```

Open a new terminal or use your browser to access the endpoint:

```bash
curl http://127.0.0.1:8080
# Output: Hello, World!
```
