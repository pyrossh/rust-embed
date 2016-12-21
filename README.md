## Rust Embed
Generates rust code to embed resource files into your rust executable

```bash
./rust-embed

rust-embed v0.1.0
Generates rust code to embed resource files/folders at compile time into your library or executable

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

Note:
 
This is not the same as std macros,
`include_bytes!`
`include_str!`
these are macros which generate code at compile time whereas rust-embed generates code when the command is run. And rust-embed works only on folders.

## Installation

```
cargo install rust-embed
```

## Documentation
It exposes a function to serve all files stored in your assets folder which is useful for webservers. So now you can statically compile all your assets i.e. your /static/ or /public/ folders into the rust executable and serve them. So now you don't need to package your assets with your executable.

```rust
assets::get(path: str) -> Result<&[u8], &str>
// This will return the data for the file specified by the file path or an error if it cannot be found.

assets::list() -> Vec<&'static str>
// This will return the list of all files which were included
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
    res.send(&assets::examples_public_index_html).unwrap();
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

# TODO
  rewrite this to use compiler plugins and macro so that we can use that instead
