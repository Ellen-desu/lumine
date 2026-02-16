# Lumine

A **synchronous HTTP web server** written in Rust.

Lumine is designed to be:

- **Fast** — Although synchronous, Lumine performs well for small to medium-scale APIs.
- **Easy** — Simple architecture, minimal concepts, and beginner-friendly API.

---

## Why Lumine?

Most modern Rust web frameworks are async-first. That’s great, but not always ideal.

Lumine exists for cases where you want:

- To **learn how an HTTP server works internally**
- A **simple web server without async/await complexity**
- Full control with minimal dependencies and a std-first design

> Lumine focuses on clarity first, performance second, and magic never.

---

## Installation

Add Lumine to your project using Cargo.

Using `Cargo.toml`:
```toml
lumine = "0.1"
````

Or via command line:

```bash
cargo add lumine
```

---

## Example

A minimal **Hello World** HTTP server using Lumine.

*main.rs*

```rust.no_run
use lumine::{Lumine, Result};
use std::net::TcpListener;

fn main() -> Result<()> {
    let app = Lumine::builder()
        .route("/", |_| "Hello, World!")
        .build();

    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // Start serving incoming connections
    let rx = app.serve(listener);
    
    // ⚠️ IMPORTANT:
    // This loop is REQUIRED.
    //
    // The receiver must be continuously polled to keep the
    // internal event loop alive. If this loop is removed,
    // the server will stop processing events properly.
    //
    // In short: no recv loop = dead server.
    while let Ok(err) = rx.recv() {
        eprintln!("Client error: {err}");
    }

    Ok(())
}
```

Then open your browser at:

```bash
http://127.0.0.1:8080
```

---

## Examples

More complete examples are available in the `/examples` directory.

To run an example:

```bash
cargo run --example <example-name>
```

---

## Behind the Scenes

Internally, Lumine is built using:

* `std::net::TcpListener` for networking
* A simple routing mechanism
* Thread-based request handling
* An event channel for propagating client-side errors

No async runtime, no hidden magic.

---

## Project Status

> ⚠️ **Early Development Warning**

Breaking changes may occur as the API evolves.

---

## License

This project is licensed under the [MIT License.](https://github.com/Ellen-desu/lumine/blob/main/LICENSE)
