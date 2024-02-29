# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2024-02-29

### Changed

- [4b5867a] Change implementation, now based on `static-regular-grammar`.

### Removed

- [4b5867a] Remove mutable methods (will be reintroduced if necessary later).

## [0.3.4] - 2023-05-22

### Added

- [37b45ba] Add `AsLanguageTag` trait.

## [0.3.3] - 2023-05-20

### Added

- [0f22312] Add `serde` support for `LanguageTagBuf`.

### Fixed

- [71a205a] Fix README links.
- [43d3a06] Fix README links.

## [0.3.2] - 2022-08-18

### Added

- [97a10ab] Add `fmt::Display` impl for `GrandfatheredTag`.

## [0.3.1] - 2022-08-18

### Added

- [eb305b0] Add inner buffer access methods.

## [0.2.0] - 2020-12-19

### Added

- [7be6201] impl copy.

### Changed

- [ccca814] Move to 2.0.0

### Fixed

- [f4d9a43] Fix LanguageTagBuf::as_ref.

## [0.1.1] - 2020-12-18

### Added

- [d1a43f3] Impl Clone.

### Changed

- [0cf935e] Refactor & Iterators.
- [638a21b] refactoring + new tests

### Fixed

- [cda890f] Fix comments.
- [c7304fb] Fix bugs + refact + preparing mut API.
- [7f247d0] Fix README ref links.

