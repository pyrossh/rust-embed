#![feature(attr_literals)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate rust_embed;

#[test]
#[cfg(debug_assertions)]
fn dev() {
  #[derive(RustEmbed)]
  #[folder("examples/public/")]
  struct Asset;

  match Asset::get("index.html") {
    None => assert!(false, "index.html should exist"),
    _ => assert!(true),
  }
  match Asset::get("gg.html") {
    Some(_) => assert!(false, "gg.html should not exist"),
    _ => assert!(true),
  }
  match Asset::get("images/llama.png") {
    None => assert!(false, "llama.png should exist"),
    _ => assert!(true),
  }
}

#[test]
#[cfg(not(debug_assertions))]
fn prod() {
  #[derive(RustEmbed)]
  #[folder("examples/public/")]
  struct Asset;

  match Asset::get("index.html") {
    None => assert!(false, "index.html should exist"),
    _ => assert!(true),
  }
  match Asset::get("gg.html") {
    Some(_) => assert!(false, "gg.html should not exist"),
    _ => assert!(true),
  }
  match Asset::get("images/llama.png") {
    None => assert!(false, "llama.png should exist"),
    _ => assert!(true),
  }
}
