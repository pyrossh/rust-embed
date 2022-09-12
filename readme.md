## Rust Embed [![Build Status](https://github.com/pyros2097/rust-embed/workflows/Test/badge.svg)](https://github.com/pyros2097/rust-embed/actions?query=workflow%3ATest) [![crates.io](https://img.shields.io/crates/v/rust-embed.svg)](https://crates.io/crates/rust-embed)

Rust Custom Derive Macro which loads files into the rust binary at compile time during release and loads the file from the fs during dev.

You can use this to embed your css, js and images into a single executable which can be deployed to your servers. Also it makes it easy to build a very small docker image for you to deploy.

## Installation

```toml
[dependencies]
rust-embed="6.4.1"
```

## Documentation

You need to add the custom derive macro RustEmbed to your struct with an attribute `folder` which is the path to your static folder.

The path resolution works as follows:

- In `debug` and when `debug-embed` feature is not enabled, the folder path is resolved relative to where the binary is run from.
- In `release` or when `debug-embed` feature is enabled, the folder path is resolved relative to where `Cargo.toml` is.

```rust
#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Asset;
```

The macro will generate the following code:

```rust
impl Asset {
  pub fn get(file_path: &str) -> Option<rust_embed::EmbeddedFile> {
    ...
  }

  pub fn iter() -> impl Iterator<Item = Cow<'static, str>> {
    ...
  }
}
impl RustEmbed for Asset {
  fn get(file_path: &str) -> Option<rust_embed::EmbeddedFile> {
    ...
  }
  fn iter() -> impl Iterator<Item = Cow<'static, str>> {
    ...
  }
}

// Where EmbeddedFile contains these fields,
pub struct EmbeddedFile {
  pub data: Cow<'static, [u8]>,
  pub metadata: Metadata,
}
pub struct Metadata {
  hash: [u8; 32],
  last_modified: Option<u64>,
}
```

### `get(file_path: &str) -> Option<rust_embed::EmbeddedFile>`

Given a relative path from the assets folder returns the `EmbeddedFile` if found.

If the feature `debug-embed` is enabled or the binary compiled in release mode the bytes have been embeded in the binary and a `Option<rust_embed::EmbeddedFile>` is returned.

Otherwise the bytes are read from the file system on each call and a `Option<rust_embed::EmbeddedFile>` is returned.

### `iter()`

Iterates the files in this assets folder.

If the feature `debug-embed` is enabled or the binary compiled in release mode a static array to the list of relative paths to the files is returned.

Otherwise the files are listed from the file system on each call.

## The `prefix` attribute

You can add `#[prefix = "my_prefix/"]` to the `RustEmbed` struct to add a prefix
to all of the file paths. This prefix will be required on `get` calls, and will
be included in the file paths returned by `iter`.

## Features

### `debug-embed`

Always embed the files in the binary, even in debug mode.

### `interpolate-folder-path`

Allow environment variables to be used in the `folder` path. Example:

```rust
#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/foo"]
struct Asset;
```

This will pull the `foo` directory relative to your `Cargo.toml` file.

### `compression`

Compress each file when embedding into the binary. Compression is done via [`include-flate`].
> Note this feature should only be used with clean builds as it doesn't work properly for incremental builds yet. Please look at this issue for more details [#182](https://github.com/pyrossh/rust-embed/issues/182)


### `include-exclude`
Filter files to be embedded with multiple `#[include = "*.txt"]` and `#[exclude = "*.jpg"]` attributes. 
Matching is done on relative file paths, via [`globset`].
`exclude` attributes have higher priority than `include` attributes.
Example:

```rust
#[derive(RustEmbed)]
#[folder = "examples/public/"]
#[include = "*.html"]
#[include = "images/*"]
#[exclude = "*.txt"]
struct Asset;
```

## Usage

```rust
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "examples/public/"]
#[prefix = "prefix/"]
struct Asset;

fn main() {
  let index_html = Asset::get("prefix/index.html").unwrap();
  println!("{:?}", std::str::from_utf8(index_html.data.as_ref()));

  for file in Asset::iter() {
      println!("{}", file.as_ref());
  }
}
```

## Integrations

1. [Poem](https://github.com/poem-web/poem) for poem framework under feature flag "embed"
2. [warp_embed](https://docs.rs/warp-embed/latest/warp_embed/) for warp framework

## Examples

To run the example in dev mode where it reads from the fs,

`cargo run --example basic`

To run the example in release mode where it reads from binary,

`cargo run --example basic --release`

Note: To run the [actix-web](https://github.com/actix/actix-web) example:

`cargo run --example actix --features actix`

Note: To run the [rocket](https://github.com/SergioBenitez/Rocket) example:

`cargo run --example rocket --features rocket`

Note: To run the [warp](https://github.com/seanmonstar/warp) example:

`cargo run --example warp --features warp-ex`

Note: To run the [axum](https://github.com/tokio-rs/axum) example:

`cargo run --example axum --features axum-ex`

Note: To run the [poem](https://github.com/poem-web/poem) example:

`cargo run --example poem --features poem-ex`

Note: To run the [salvo](https://github.com/salvo-rs/salvo) example:

`cargo run --example salvo --features salvo-ex`

## Testing

debug: `cargo test --test lib`

release: `cargo test --test lib --release`

Go Rusketeers!
The power is yours!

[`include-flate`]: https://crates.io/crates/include-flate
[`globset`]: https://crates.io/crates/globset
