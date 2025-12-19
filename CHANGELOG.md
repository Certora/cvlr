# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] ReleaseDate
  ### Fixed
    - `NativeFixed::to_bits` overly conservative assumption
  ### Added
    - `cvlr_log_impl!` macro to help implement `CvlrLog` trait
    - `cvlr_assume_XXX!` macros to match `cvlr_assert_XXX`
    - more local tests
  ### Changed
    - removed duplicate definitions in cvlr-asserts

## [0.4.1] - 2025-05-14

### Added
  - Allow extra comma at the end of clog! macro
  - cvlr-fixed library supports div and ceil
  - cvlr-fixed numbers are logged as decimals
  - Source location added for rule attribute
  - Support for scopes in cvlr-log
### Changed
  - NativeInt are passed by value internally
### Removed


## [0.4.0] - 2025-03-17

### Added
  - Logging of i128 and u128 integers
  - Release package on crates.io

### Changed

### Removed

## [0.3.2] - 2025-02-01

### Added
  - This is the first official release

### Fixed

### Changed

### Removed

<!-- next-url -->
[Unreleased]: https://github.com/Certora/cvlr/compare/v0.4.1...HEAD
[0.4.1]: https://github.com/Certora/cvlr/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/Certora/cvlr/compare/v0.3.2...v0.4.0
[0.3.2]: https://github.com/Certora/cvlr/releases/tag/v0.3.2