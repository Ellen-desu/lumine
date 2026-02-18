# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.2] - 2026-02-18

### Removed
- Removed manual installation using `Cargo.toml` directly to ensures users will install the latest version.

## [0.2.1] - 2026-02-18

### Added
- Added IntoResponse implementations for (u16, Body), (u16, HeaderMap), and (u16, HeaderMap, Body).

### Changed
- Improved the quality of the crate usage examples by adding some documentation and refactoring the code.

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
- Fixed malformed `CHANGELOG.md`.

### Changed
- **Breaking:** Changed the return of `Lumine::serve()` from `Result<Receiver<Error>>` to `Receiver<Client<Ready>>`.
- Switch the panic logic if the workers is set to zero from `Lumine::serve()` to `Lumine::set_workers()`.
- Renamed `services` directory to `internal`.
- Moved all test files to the root of `tests/` directory.

### Removed
- **Breaking:** Removed some unused errors in `Error` struct.
- Removed some development plans to focus on stability in this crate

## [0.1.1] - 2026-02-08

### Fixed
- Fixed inability to decode percent characters in query parameters.

### Changed
- Changed query parameters decoder to `form_urlencoded` instead of manual parsing.
- Changed Lumine development plan in `README.md`.
- Changed the structure of test module.

### Added
- Added `form_urlencoded` dependencies.
- Added more test cases to test routing, response, and parameters.

## [0.1.0] - 2026-02-04

### Added
- Initial release.
