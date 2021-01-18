# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

Thanks to [Mcat12](https://github.com/Mcat12) for the changelog.

## [5.9.0] - 2021-01-18

### Added

- Added path prefix attribute

## [5.8.0] - 2021-01-06

### Fixed

- Fixed compiling with latest version of syn

## [5.7.0] - 2020-12-08

### Fixed

- Fix feature flag typo

## [5.6.0] - 2020-07-19

### Fixed

- Fixed windows path error in release mode

### Changed

- Using github actions for CI now

## [5.5.1] - 2020-03-19

### Fixed

- Fixed warnings in latest nightly

## [5.5.0] - 2020-02-26

### Fixed

- Fixed the `folder` directory being relative to the current directory.
  It is now relative to `Cargo.toml`.

## [5.4.0] - 2020-02-24

### Changed

- using rust-2018 edition now
- code cleanup
- updated examples and crates

## [5.3.0] - 2020-02-15

### Added

- `compression` feature for compressing embedded files

## [5.2.0] - 2019-12-05

## Changed

- updated syn and quote crate to 1.x

## [5.1.0] - 2019-07-09

## Fixed

- error when debug code tries to import the utils crate

## [5.0.1] - 2019-07-07

## Changed

- derive is allowed only on unit structs now

## [5.0.0] - 2019-07-05

## Added

- proper error message stating only unit structs are supported

## Fixed

- windows latest build

## [4.5.0] - 2019-06-29

## Added

- allow rust embed derive to take env variables in the folder path

## [4.4.0] - 2019-05-11

### Fixed

- a panic when struct has doc comments

### Added

- a warp example

## [4.3.0] - 2019-01-10

### Fixed

- debug_embed feature was not working at all

### Added

- a test run for debug_embed feature

## [4.2.0] - 2018-12-02

### Changed

- return `Cow<'static, [u8]>` to preserve static lifetime

## [4.1.0] - 2018-10-24

### Added

- `iter()` method to list files

## [4.0.0] - 2018-10-11

### Changed

- avoid vector allocation by returning `impl AsRef<[u8]>`

## [3.0.2] - 2018-09-05

### Added

- appveyor for testing on windows

### Fixed

- handle paths in windows correctly

## [3.0.1] - 2018-07-24

### Added

- panic if the folder cannot be found

## [3.0.0] - 2018-06-01

### Changed

- The derive attribute style so we don't need `attr_literals` and it can be used in stable rust now. Thanks to [Mcat12](https://github.com/Mcat12).

```rust
#[folder("assets/")]
```

to

```rust
#[folder = "assets/"]
```

### Removed

- log dependecy as we are not using it anymore

## [2.0.0] - 2018-05-26

### Changed

- Reimplemented the macro for release to use include_bytes for perf sake. Thanks to [lukad](https://github.com/lukad).

## [1.1.1] - 2018-03-19

### Changed

- Fixed usage error message

## [1.1.0] - 2018-03-19

### Added

- Release mode for custom derive

### Changed

- Fixed tests in travis

## [1.0.0] - 2018-03-18

### Changed

- Converted the rust-embed macro `embed!` into a Rust Custom Derive Macro `#[derive(RustEmbed)]` which implements get on the struct

```rust
let asset = embed!("examples/public/")
```

to

```rust
#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Asset;
```

## [0.5.2] - 2018-03-16

### Added

- rouille example

## [0.5.1] - 2018-03-16

### Removed

- the plugin attribute from crate

## [0.5.0] - 2018-03-16

### Added

- rocket example

### Changed

- Converted the rust-embed executable into a macro `embed!` which now loads files at compile time during release and from the fs during dev.

## [0.4.0] - 2017-03-2

### Changed

- `generate_assets` to public again

## [0.3.5] - 2017-03-2

### Added

- rust-embed prefix to all logs

## [0.3.4] - 2017-03-2

### Changed

- the lib to be plugin again

## [0.3.3] - 2017-03-2

### Changed

- the lib to be proc-macro from plugin

## [0.3.2] - 2017-03-2

### Changed

- lib name from `rust-embed` to `rust_embed`

## [0.3.1] - 2017-03-2

### Removed

- hyper example

## [0.3.0] - 2017-02-26

### Added

- rust-embed executable which generates rust code to embed resource files into your rust executable
  it creates a file like assets.rs that contains the code for your assets.

## [0.2.0] - 2017-03-16

### Added

- rust-embed executable which generates rust code to embed resource files into your rust executable
  it creates a file like assets.rs that contains the code for your assets.
