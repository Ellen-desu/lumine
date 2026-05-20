# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Request headers limiter.
- File sending in response.
- `CONTRIBUTING.md` file.
- Asynchronous runtime with Tokio.

## [0.4.4] - 2026-05-18

### Changed
- Reuse single buffer to read request line and headers.

## [0.4.3] - 2026-05-18

### Added
- `BodyTooLarge` & `HeadersTooLarge` enum value to `Error`.
- Benchmark to all parsers.

### Fixed
- Read body content that exceeded the limit.

### Changed
- `parse_body` parameters.

## [0.4.2] - 2026-05-16

### Added
- `Middleware` documentation.

### Fixed
- Comment didn't match with the function return.
- Missing `Added` section in 0.4.1 changelog.

### Changed
- `IntoBody` return to type alias `Body`.

## [0.4.1] - 2026-04-29

### Added
- `Middleware` unit test.

## [0.4.0] - 2026-03-23

### Added
- Middleware feature.

### Changed
- `lib.rs` now re-exporting from `mod.rs`.
- Using `self` keyword for re-exporting make code concisely.
- Moved all routing traits to `routing` module.

### Removed
- `traits` module.

## [0.3.1] - 2026-03-18

### Removed
- `RouteType` alias.

### Changed
- Renamed callback attribute at `Route` struct.
- Dependencies versioning to 2 digits.

## [0.3.0] - 2026-03-02

### Added
- `should_server_close` and `set_connection_header` support function for handler.

### Removed
- `threadpool` dependency.
- *BREAKING:* Workers configuration.

### Fixed
- Thread starvation on `keep-alive` connections.

### Changed
- The key is entered when retrieving `content-length` header value.
- `routing` example application build flow to prevent breaking change.

## [0.2.4] - 2026-02-27

### Removed
- `ureq` dependency on benchmark.
- `hello_world` benchmark.

### Fixed
- Dropping client connections even though the `keep-alive` header is set.

## [0.2.3] - 2026-02-21

### Removed
- Type states at `Client` struct.

## [0.2.2] - 2026-02-18

### Removed
- Manual installation using `Cargo.toml` directly to ensures users will install the latest version.

## [0.2.1] - 2026-02-18

### Added
- `IntoResponse` implementations for `(u16, Body)`, `(u16, HeaderMap)`, and `(u16, HeaderMap, Body)`.

### Fixed
- 500 response occurs when the handler returns a status code above 500 but still below 600.

### Removed
- Error body response when the handler returns an invalid status code.

## [0.2.0] - 2026-02-15

### Added
- `Client` struct for accessing the request sender information.
- Timeout configuration when reading and writing data at the stream using `Lumine::set_timeout()`.
- 500 status code response when the route/worker is panicked.

### Fixed
- Malformed `CHANGELOG.md`.

### Changed
- **Breaking:** Changed the return of `Lumine::serve()` from `Result<Receiver<Error>>` to `Receiver<Client<Ready>>`.
- Switch the panic logic if the workers is set to zero from `Lumine::serve()` to `Lumine::set_workers()`.
- Renamed `services` directory to `internal`.
- Moved all test files to the root of `tests/` directory.

### Removed
- **Breaking:** Removed some unused errors in `Error` struct.
- All development plans to focus on stability in this crate.

## [0.1.1] - 2026-02-08

### Fixed
- Inability to decode percent characters in query parameters.

### Changed

- Query parameters decoder to `form_urlencoded` instead of manual parsing.
- Lumine development plan in `README.md`.
- Structure of test module.

### Added

- `form_urlencoded` dependency.
- More test cases to test routing, response, and parameters.

## [0.1.0] - 2026-02-04

### Added
- Initial release.
