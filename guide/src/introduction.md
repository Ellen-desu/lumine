# Introduction

Welcome to the official **Lumine** guide!

Lumine is an asynchronous HTTP web server framework written in **Rust**. It is designed with a focus on simplicity, high performance, and an ergonomic developer experience.

## Why Lumine?

- **Asynchronous by Default**: Built on top of the `tokio` runtime, Lumine can handle thousands of concurrent connections with minimal overhead.
- **Ergonomic & Simple**: Provides a clean and intuitive API, similar to modern frameworks in other languages (like Express.js in Node or Axum in the Rust ecosystem).
- **Type-Safe**: Leverages Rust's type system to ensure request validation and response generation are verified at compile time.
- **Extensible**: Supports a powerful Middleware system to easily add cross-cutting features like logging, authentication, CORS, and global error handling.

## Key Features

1. **Dynamic Routing**: Supports static routes, dynamic path parameters (`/users/:id`), and query parameters (`?sort=date`).
2. **Header & Body Inspection**: Easy data extraction from HTTP requests, including automatic JSON parsing via `serde`.
3. **Ergonomic Response Conversion**: The `IntoResponse` trait allows returning various types (such as `&str`, `String`, `StatusCode`, and tuples) directly from your route handlers.
4. **File Streaming**: Support for memory-efficient file delivery using the built-in `filestream` feature.
