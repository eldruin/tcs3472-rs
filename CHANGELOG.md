# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate

### Added
- Async support based on `embedded-hal-async` 1.0.
- Derive `Eq`, `Hash` for simple types.

### Changed
- Raise MSRV to 1.81.0.
- Update to embedded-hal 1.0

## [0.2.0] - 2021-04-05

### Added
- Reading the measurement of all channels at once.
- Make types copyable, clonable, comparable and debuggable where possible.

### Changed
- Removed `Default` derive for sensor struct.

## [0.1.1] - 2018-10-21

This crate is now functionally complete.

### Added
- Setting the RGB converter gain.
- Enabling/disabling the RGB converter interrupt generation.
- Setting the RGB converter interrupt clear channel low/high thresholds.
- Setting the RGB converter interrupt persistence.
- Setting the number of integration cycles.
- Reading the device ID.
- Enabling/disabling the wait feature.
- Setting the number of wait time cycles.
- Enabling/disabling the *wait long* setting.

### Fixed
- Fixed selecting auto-increment command mode for multiple register reading.

## 0.1.0 - 2018-10-20

This is the initial release to crates.io. All changes will be documented in
this CHANGELOG.


<!-- next-url -->
[Unreleased]: https://github.com/eldruin/tcs3472-rs/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/eldruin/tcs3472-rs/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/eldruin/tcs3472-rs/compare/v0.1.0...v0.1.1
