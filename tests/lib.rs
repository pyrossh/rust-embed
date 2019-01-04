#[macro_use]
extern crate rust_embed;

/// Test doc comment
#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Asset;

#[test]
fn get_works() {
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
fn iter_works() {
  let mut num_files = 0;
  for file in Asset::iter() {
    assert!(Asset::get(file.as_ref()).is_some());
    num_files += 1;
  }
  assert_eq!(num_files, 6);
}

#[test]
fn trait_works_generic() {
  trait_works_generic_helper(&Asset);
}
fn trait_works_generic_helper(folder: &impl rust_embed::RustEmbed) {
  let mut num_files = 0;
  for file in folder.iter() {
    assert!(folder.get(file.as_ref()).is_some());
    num_files += 1;
  }
  assert_eq!(num_files, 6);
  assert!(folder.get("gg.html").is_none(), "gg.html should not exist");
}

#[test]
fn trait_works_object() {
  trait_works_object_helper(&Asset);
}
fn trait_works_object_helper(folder: &dyn rust_embed::RustEmbed) {
  let mut num_files = 0;
  for file in folder.iter() {
    assert!(folder.get(file.as_ref()).is_some());
    num_files += 1;
  }
  assert_eq!(num_files, 6);
  assert!(folder.get("gg.html").is_none(), "gg.html should not exist");
}
