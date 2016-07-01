## Rust Embed
Generates rust code to embed resource files into your rust executable

# TODO
  rewrite this to use compiler plugins and macro so that we can use that instead

```bash
./rust-embed

rust-embed v0.1.0
Generates rust code to embed resource files into your library or executable

  Usage:
    rust-embed input_folder output_file

  where:
    input_folder   string is the path to the folder containing the assets.
    output_file    string is output filename.

  example:
    rust-embed src/public src/assets.rs
```

You can use this to embed your css, js and images into a single executable.

This is similar to [go-bindata](https://github.com/jteeuwen/go-bindata).

This is similar to [pony-embed](https://github.com/pyros2097/pony-embed).

## Installation

> **Note:** this method currently requires you to be running cargo 0.6.0 or
> newer.

```
cargo install --git https://github.com/pyros2097/rust-embed
```

or if you're using [`multirust`](https://github.com/brson/multirust)

```
multirust run nightly cargo install --git https://github.com/pyros2097/rust-embed
```


## Documentation
First make sure you've got Rust **1.4.0** or greater available.

You can directly access your files as constants from the assets module or
you can use this function to serve all files stored in your assets folder which might be useful for webservers.

```rust
assets::index_html // direct access

assets::get(name: str)  
// This will return the data for the specified resource name or will throw an error if it cannot be found.
```

## Examples
A simple http server which serves its resources directly.

To compile the assets for the examples,
`rust-embed examples/public/ examples/assets.rs`

To run the examples,
`cargo run --example http`

```rust
extern crate hyper;
use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::net::Fresh;

mod assets;

fn handle_index(_: Request, res: Response<Fresh>) {
    res.send(&assets::index_html).unwrap();
    // or
    // res.send(assets::get("examples/public/index.html").unwrap()).unwrap();
}

fn main() {
  println!("Server running on 127.0.0.1:3000");
  Server::http("127.0.0.1:3000").unwrap().handle(handle_index).unwrap();
}
```

Go Rusketeers!
The power is yours!
