# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

Thanks to [Mcat12](https://github.com/Mcat12) for the changelog.

## [2.0.0] - 2018-05-26
### Changed
- Reimplemented the macro for release to use include_bytes for perf sake. Thanks to [lukad](https://github.com/lukad).

## [1.1.1] - 2018-03-19
### Changed
- Converted the rust-embed macro `embed!` into a Rust Custom Derive Macro `#[derive(RustEmbed)]`

## [0.5.0] - 2018-03-16
### Changed
- Converted the rust-embed executable into a macro `embed!` which now loads files at compile time during release and from the fs during dev.

## [0.2.0] - 2017-03-16
### Added
- rust-embed executable which generates rust code to embed resource files into your rust executable
  it creates a file like assets.rs that contains the code for your assets.
