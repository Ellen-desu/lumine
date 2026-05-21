# Contributing to Lumine

First off, thank you for considering contributing to Lumine! It's people like you who make Lumine a great tool for the Rust community.

Lumine is designed to be fast, simple, and flexible. We value clarity and safety above all else.

## Coding Standards

To maintain the quality and safety of the codebase, please adhere to the following guidelines:

### 1. Safety First
* **No `unsafe`**: Lumine aims to be 100% safe Rust. Avoid `unsafe` blocks unless there is an absolute, performance-critical necessity that has been thoroughly discussed.
* **Avoid `.unwrap()` and `.expect()`**: Always handle potential errors gracefully using `Result` or `Option` with patterns like `?`, `match`, or `if let`. We want to avoid runtime panics whenever possible.
* **No `panic!`**: Use the internal `Error` type to propagate errors back to the user or the runtime.

### 2. Idiomatic Rust
* Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).
* Use `cargo fmt` to ensure consistent formatting.
* Use `cargo clippy` to catch common mistakes and improve code quality.

### 3. Documentation
* Ensure all public APIs are documented using triple-slash (`///`) comments.
* Provide examples for new features in the documentation or the `examples/` directory.

### 4. Testing
* Include unit tests for new logic.
* If you're fixing a bug, add a regression test in the `tests/` directory.
* Run all tests with `cargo test` before submitting your PR.

## How Can I Contribute?

### Reporting Bugs
* Use the GitHub Issue Tracker.
* Describe the bug in detail, including steps to reproduce and the expected vs. actual behavior.
* Include your environment details (OS, Rust version).

### Suggesting Enhancements
* Open an issue to discuss the enhancement before starting implementation.
* Explain the use case and why this change would benefit Lumine users.

### Pull Requests
1. Fork the repository and create your branch from `main`.
2. Ensure your code follows the coding standards mentioned above.
3. Update the `CHANGELOG.md` if your changes are significant.
4. Submit the PR with a clear description of what you've changed and why.

## License

By contributing to Lumine, you agree that your contributions will be licensed under the project's [MIT License](LICENSE).
