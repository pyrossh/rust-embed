## Rust Embed [![Build Status](https://travis-ci.org/pyros2097/rust-embed.svg?branch=master)](https://travis-ci.org/pyros2097/rust-embed) [![crates.io](http://meritbadge.herokuapp.com/rust-embed)](https://crates.io/crates/rust-embed)
Rust Marco which loads files into the rust binary at compile time during release and loads the file from the fs during dev.

You can use this to embed your css, js and images into a single executable.

This is similar to [go-embed](https://github.com/pyros2097/go-embed).

This is similar to [pony-embed](https://github.com/pyros2097/pony-embed).

Note:
 
This is not the same as std macros,
`include_bytes!`
`include_str!`
these are macros which generate code at compile time for only files.
`embed!("examples/public/")` accepts folders and returns a function to get the contents using the file path

## Installation

```
[dependencies]
rust-embed="0.5.2"
```

## Documentation
The `embed!` macro takes a folder path and returns a function which allows you to get the file by passing the file path within the folder. So now you can statically compile all your assets i.e. your /static/ or /public/ folders into the rust executable and serve them during release and in development it will load the file from the file
system so that it doesn't take to much time to compile;]

```rust
asset(path: String) -> Option<Vec<u8>>
```

## Usage
```rust
#[macro_use]
extern crate rust_embed;

use rust_embed::*;

fn main() {
  let asset = embed!("examples/public/".to_owned());
  let index_html = asset("index.html".to_owned()).unwrap();
  println!("{}", index_html);
}
```

## Examples
To run the example in dev mode where it reads from the fs,

`cargo run --example rocket`

To run the example in release mode where it reads from binary,

`cargo run --release --example rocket`
## Testing
debug: `cargo test --lib`

release: `cargo test --lib --release`

Go Rusketeers!
The power is yours!
