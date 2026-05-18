# Lumine

A lightweight HTTP web server framework written in Rust.

Lumine is designed to be:

* **Fast** — Minimal overhead with a focus on practical performance.
* **Simple** — Clean architecture, minimal concepts, and beginner-friendly API.
* **Flexible** — Designed to evolve without unnecessary abstractions.

---

## Why Lumine?

Many Rust web frameworks introduce heavy abstractions or runtime complexity early.

Lumine exists for cases where you want:

* To **learn how HTTP servers work internally**
* A **minimal and understandable architecture**
* Full control with a std-first design philosophy
* A framework that stays close to how Rust actually works

> Lumine focuses on clarity first, performance second, and magic never.

---

## Installation

Add Lumine to your project via command line.

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

Internally, Lumine currently uses:

* `std::net::TcpListener` for networking
* A lightweight routing mechanism
* Internal request handling abstractions
* An event channel for propagating client-side errors

The architecture may continue evolving as the project grows.

---

## Project Status

> ⚠️ **Early Development Warning**

Breaking changes may occur as the API evolves.

---

## License

This project is licensed under the [MIT License](https://github.com/Ellen-desu/lumine/blob/main/LICENSE).
