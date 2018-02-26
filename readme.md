## Rust Embed
Rust Marco which loads files into the rust binary at compile time during release and loads the file from the fs during dev.

You can use this to embed your css, js and images into a single executable.

This is similar to [go-embed](https://github.com/pyros2097/go-embed).

This is similar to [pony-embed](https://github.com/pyros2097/pony-embed).

Note:
 
This is not the same as std macros,
`include_bytes!`
`include_str!`
these are macros which generate code at compile time for only files.
`embed!("examples/public")` accepts folders and returns a function to get the contents using the file path

## Installation

```
[dependencies]
rust-embed="0.3.0"
```

## Documentation
It exposes a function to serve all files stored in your assets folder which is useful for webservers. So now you can statically compile all your assets i.e. your /static/ or /public/ folders into the rust executable and serve them during release and in development it will load the file from the file
system so that it doesn't take to much time to compile;]

```rust
asset(path: String) -> Option<String>
```

## Examples
To run the examples,
`cargo run --example hyper`

Go Rusketeers!
The power is yours!
