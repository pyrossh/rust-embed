## Rust Embed [![Build Status](https://travis-ci.org/pyros2097/rust-embed.svg?branch=master)](https://travis-ci.org/pyros2097/rust-embed) [![Build status](https://ci.appveyor.com/api/projects/status/jnr45fn1l1dop2a8/branch/master?svg=true)](https://ci.appveyor.com/project/pyros2097/rust-embed/branch/master) [![crates.io](http://meritbadge.herokuapp.com/rust-embed)](https://crates.io/crates/rust-embed)
Rust Custom Derive Macro which loads files into the rust binary at compile time during release and loads the file from the fs during dev.

You can use this to embed your css, js and images into a single executable which can be deployed to your servers. Also it makes it easy to build a very small docker image for you to deploy.

### Dev
<img src="https://user-images.githubusercontent.com/1687946/40840773-b1ae1ce6-65c5-11e8-80ac-9e9196701ca2.png" width="700" height="100">

### Release
<img src="https://user-images.githubusercontent.com/1687946/40840774-b1dd709a-65c5-11e8-858d-73a88e25f07a.png" width="700" height="184">

## Installation

```
[dependencies]
rust-embed="3.0.0"
```

## Documentation
You need to add the custom derive macro RustEmbed to your struct with an attribute `folder` which is the path to your static folder.
```rust
#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Asset;
```
This macro add a single static method `get` to your type. This method allows you to get your assets from the fs during dev and from the binary during release. It takes the file path as string and returns an an optional vector of u8.
```rust
pub fn get(file_path: &str) -> Option<Vec<u8>>
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

`cargo run --example basic --release`

Note: To run the `actix-web` example:

`cargo run --example actix --features actix`

Note: To run the `rocket` example, add the `nightly` feature flag and run on a nightly build:

`cargo +nightly run --example rocket --features nightly`

## Testing
debug: `cargo test --test lib`

release: `cargo test --test lib --release`

Go Rusketeers!
The power is yours!
