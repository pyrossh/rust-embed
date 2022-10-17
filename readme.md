# Rust Embed for Web

Rust Macro which embeds files into your executable. A fork of `rust-embed` with a focus on usage in web servers.

## Differences from `rust-embed`

This crate opts to make some choices that may increase the size of your
executable in exchange for better performance at runtime. In particular:

- Contents of the file are stored twice, both gzipped and regular. This makes it
  possible to serve files from a server, depending on whether the client accepts
  compression or not, without having to compress or decompress anything at
  runtime.
  - If the compression makes little difference, for example a jpeg file won't
    compress much further if at all, then the compressed version is skipped.
  - You can also disable this behavior by adding an attribute `#[gzip = false]`.
    When disabled, the compressed files won't be included for that embed.
- Some metadata that is useful for web headers like `ETag` and `Last-Modified`
  are computed ahead of time and embedded into the executable. This makes it
  possible to use these in a web server without any computation at runtime.
- The file data is returned as a `&'static` reference. This makes is easy to use
  the file data in a server response without creating copies or reference
  counting.
  - As a result, this fork doesn't have the "debug mode" feature that
    dynamically looks up files, that just wouldn't be static! You will need to
    recompile even during debugging to update embedded files.

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
}
```

The path resolution for the `folder` is resolved relative to where `Cargo.toml` is.

### The `prefix` attribute

You can add `#[prefix = "my_prefix/"]` to the `RustEmbed` struct to add a prefix
to all of the file paths. This prefix will be required on `get` calls.

### The `gzip` attribute

You can add `#[gzip = false]` to the `RustEmbed` struct to disable gzip
compression for the files in that embed. Only files where the compression
reduces the file size is compressed already; but you can completely disable gzip
compression if you are concerned with file sizes.

## Features

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
