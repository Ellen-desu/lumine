<p align="center">
  <img src="https://raw.githubusercontent.com/Ellen-desu/lumine/refs/heads/main/assets/lumine.png" width="150">
</p>

<h1 align="center">Lumine</h1>

<p align="center">
    A lightweight asynchronous HTTP framework built with Rust and Tokio.
</p>

<p align="center">
    <a href="https://docs.rs/lumine/latest/lumine">Documentation</a> | <a href="https://github.com/Ellen-desu/lumine/blob/main/CONTRIBUTING.md">Contributing</a>
</p>

---

Lumine is designed to be:

* **Fast** — Minimal overhead with efficient async I/O.
* **Simple** — Small API surface and easy-to-follow architecture.
* **Flexible** — Build applications without fighting framework abstractions.

---

## Why Lumine?

Many Rust web frameworks provide powerful features, but can introduce significant complexity.

Lumine aims to provide:

* A lightweight and approachable HTTP framework
* A straightforward async-first architecture
* Full control over request handling and responses
* An API that stays close to idiomatic Rust

---

## Minimal Usage

```rust,ignore
use lumine::prelude::*;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Lumine::builder()
        .route("/", async |_| "Hello, World!")
        .build();

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    app.serve(listener).await;

    Ok(())
}
```

---

## Request Handlers

Handlers receive a `Request` and return any type implementing `IntoResponse`.

Inline async closure:

```rust
use lumine::prelude::*;

let app = Lumine::builder()
    .route("/", async |request: Request| {
        format!("Requested: {}", request.uri().path())
    })
    .build();
```

Or use a dedicated async function:

```rust
use lumine::prelude::*;

async fn index(request: Request) -> impl IntoResponse {
    format!("Requested: {}", request.uri().path())
}

let app = Lumine::builder()
    .route("/", index)
    .build();
```

---

## Responses

Anything that implements [`IntoResponse`](https://docs.rs/lumine/latest/lumine/trait.IntoResponse.html) can be returned from handlers.

```rust
use lumine::prelude::*;

async fn plain_text(_: Request) -> &'static str {
    "Hello, World!"
}

async fn json(_: Request) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
    
    (StatusCode::CREATED, headers, "Hello, World!")
}

let app = Lumine::builder()
    .route("/", plain_text)
    .route("/json", json)
    .build();
```

---

## Examples

More examples can be found in the `examples/` directory.

Run an example:

```bash
cargo run --example <example-name>
```

---

## Project Status

> ⚠️ Early Development

Lumine is actively evolving and breaking changes may occur between releases.

Feedback, issues, and contributions are welcome.

---

## License

Licensed under the [MIT License](https://github.com/Ellen-desu/lumine/blob/main/LICENSE).
