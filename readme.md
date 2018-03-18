## Rust Embed [![Build Status](https://travis-ci.org/pyros2097/rust-embed.svg?branch=master)](https://travis-ci.org/pyros2097/rust-embed) [![crates.io](http://meritbadge.herokuapp.com/rust-embed)](https://crates.io/crates/rust-embed)
Rust Custom Derive Macro which loads files into the rust binary at compile time during release and loads the file from the fs during dev.

You can use this to embed your css, js and images into a single executable.

This is similar to [go-embed](https://github.com/pyros2097/go-embed).

This is similar to [pony-embed](https://github.com/pyros2097/pony-embed).

## Installation

```
[dependencies]
rust-embed="1.1.0"
```

## Documentation
Declare a struct name it Asset or something and add an attribute `folder` to it which has the path to your static folder. 
```rust
#![feature(attr_literals)]

#[derive(RustEmbed)]
#[folder("examples/public/")]
struct Asset;
```

## Usage
```rust
#![feature(attr_literals)]
#[macro_use]
extern crate rust_embed;
#[macro_use]
extern crate log;

#[derive(RustEmbed)]
#[folder("examples/public/")]
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
