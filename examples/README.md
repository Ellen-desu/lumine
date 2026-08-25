# Lumine Examples

Welcome to the **Lumine** examples directory! This folder contains a step-by-step learning path to help you understand how to build web applications using the Lumine HTTP framework.

Whether you are a beginner to Rust web frameworks or just looking to learn Lumine's API, these examples are designed to be easy to read and self-contained.

## 🚀 Learning Path

We recommend going through the examples in order, as they build upon each other:

| Example | Description | Run Command |
|---------|-------------|-------------|
| **[1. Hello World](1_hello_world.rs)** | The absolute basics. Learn how to create a server and respond with simple text. | `cargo run --example hello_world` |
| **[2. Routing](2_routing.rs)** | Learn how to define different routes, organize endpoints, and use different HTTP methods. | `cargo run --example routing` |
| **[3. Responses](3_response.rs)** | Returning data to the user. Covers HTML, JSON, plain text, custom headers, and status codes. | `cargo run --example response` |
| **[4. Requests](4_request.rs)** | Reading incoming data. Learn how to inspect headers, methods, and read the request body. | `cargo run --example request` |
| **[5. Parameters](5_parameters.rs)** | Dynamic routes! Extract path variables (e.g., `/users/:id`) and query strings (e.g., `?search=rust`). | `cargo run --example parameters` |
| **[6. Middleware](6_middleware.rs)** | Run code before or after requests. Great for logging, authentication, or modifying headers. | `cargo run --example middleware` |
| **[7. TLS (HTTPS)](7_tls.rs)** | Secure your server. Learn how to serve your Lumine app over HTTPS using rustls. | `cargo run --example tls` |
| **[8. Static Files](8_static_files.rs)** | Serve assets like images, CSS, and JS using `Remainder` (wildcard routes) and `FileStream`. | `cargo run --example static_files` |

## 💡 How to Run an Example

You can run any example from the root of the workspace or inside the `examples/` directory using Cargo:

```bash
cargo run --example <name_of_example>
```

For example, to run the Static Files example:

```bash
cargo run --example static_files
```

Most examples will start a server on `http://127.0.0.1:8080`. You can test them using your web browser or a tool like `curl`:

```bash
curl http://127.0.0.1:8080/
```

## 📦 Features Showcased

Some examples require specific Cargo features to be enabled (these are already enabled for the `examples` crate in `Cargo.toml`):
- `filestream`: Required for streaming files (used in `3_response.rs` and `8_static_files.rs`).
- `tls`: Required for HTTPS support (used in `7_tls.rs`).

Happy coding! If you find any issues with the examples, feel free to open a PR.
