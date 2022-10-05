# Rust Embed for Web [![Build Status](https://github.com/SeriousBug/rust-embed-for-web/workflows/Test/badge.svg)](https://github.com/SeriousBug/rust-embed-for-web/actions?query=workflow%3ATest) [![crates.io](https://img.shields.io/crates/v/rust-embed-for-web.svg)](https://crates.io/crates/rust-embed-for-web)

Rust Macro which embeds files into your executable. A fork of `rust-embed` with a focus on usage on web servers.

## Differences from `rust-embed`

This crate opts to make some choices that may increase the size of your
executable in exchange for better performance at runtime. In particular:

- Contents of the file are stored twice, both gzipped and regular. This makes it
  possible to serve files from a server, depending on whether the client accepts
  compression or not, without having to compress or decompress anything at
  runtime.
- Some metadata that is useful for web headers like `ETag` and `Last-Modified`
  are computed ahead of time and embedded into the executable. This makes it
  possible to use these in a web server without any computation at runtime.

These differences can be useful for web servers, with the caveat that it will
increase executable size beyond what the original `rust-embed` does. If you are
not building a web server, or the size of the executable is important to you,
you should likely use the original project instead.

## Installation

```toml
[dependencies]
rust-embed-for-web="7.0"
```

## Usage

To use this macro, add an empty struct, then add the derive. Then, you specify the folder to use.

```rust
#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Asset;

fn main() {
  let index_html = Asset::get("path/index.html").unwrap();
  println!("{:?}", std::str::from_utf8(index_html.data.as_ref()));

  for file in Asset::iter() {
      println!("{:?}", file.as_ref());
  }
}
```

The path resolution for the `folder` works as follows:

- In a `release` build, or when `debug-embed` feature is enabled, the folder path is resolved relative to where `Cargo.toml` is.
- Otherwise, the folder path is resolved relative to where the binary is run from.

### The `prefix` attribute

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
use rust_embed_for_web::RustEmbed;

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
