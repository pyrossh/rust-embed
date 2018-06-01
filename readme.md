## Rust Embed [![Build Status](https://travis-ci.org/pyros2097/rust-embed.svg?branch=master)](https://travis-ci.org/pyros2097/rust-embed) [![crates.io](http://meritbadge.herokuapp.com/rust-embed)](https://crates.io/crates/rust-embed)
Rust Custom Derive Macro which loads files into the rust binary at compile time during release and loads the file from the fs during dev.

You can use this to embed your css, js and images into a single executable which can be deployed to your servers. Also it makes it easy to build a very small docker image for you to deploy.

## Installation

```
[dependencies]
rust-embed="2.0.0"
```

## Documentation
Declare a struct name it Asset or something and add an attribute `folder` to it which has the path to your static folder.
```rust
#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Asset;
```

## Usage
```rust
#[macro_use]
extern crate rust_embed;

#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Asset;

fn main() {
  let index_html = Asset::get("index.html").unwrap();
  println!("{:?}", std::str::from_utf8(&index_html));
}
```

## Examples
To run the example in dev mode where it reads from the fs,

`cargo run --example basic`

To run the example in release mode where it reads from binary,

`cargo run --release --example basic`
## Testing
debug: `cargo test --tests --lib`

release: `cargo test --tests --lib --release`

Go Rusketeers!
The power is yours!
