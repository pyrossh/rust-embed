#[macro_use]
extern crate rust_embed;

#[test]
#[cfg(debug_assertions)]
fn dev() {
  #[derive(RustEmbed)]
  #[folder = "examples/public/"]
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

  let mut num_files = 0;
  for file in Asset::iter() {
    assert!(Asset::get(file.as_ref()).is_some());
    num_files += 1;
  }
  assert_eq!(num_files, 6);
}

#[test]
#[cfg(not(debug_assertions))]
fn prod() {
  #[derive(RustEmbed)]
  #[folder = "examples/public/"]
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

  let mut num_files = 0;
  for file in Asset::iter() {
    assert!(Asset::get(file.as_ref()).is_some());
    num_files += 1;
  }
  assert_eq!(num_files, 6);
}
