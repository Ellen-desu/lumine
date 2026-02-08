# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
- Added some test cases to test error handling.

## [0.1.0] - 2026-02-04

### Added
- Initial release.

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
