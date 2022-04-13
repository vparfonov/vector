# Changelog
Notes significant changes to human_bytes

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1] 2022-02-22
### Changed

* Updated `lexical` to v6.0

## [0.3] 2021-04-20
### Fixed
* Switch from using `format!` to `String.push` and `String.push_str`. +/- 8% performance improvement.

### Added
* A `bibytes` feature, which allows users to use KiB, GiB, etc instead of KB, GB, etc.
* More `crates.io` keywords to `Cargo.toml`

## [0.2.1] 2020-04-08
### Fixed
* The README example on how to enable the `fast` feature

### Changed
* Simplified the README

## [0.2] 2020-02-15
### Added
* A `fast` feature, which improves performance by using [lexical](https://github.com/Alexhuszagh/rust-lexical) instead of `format!` to convert `f64`s to strings
