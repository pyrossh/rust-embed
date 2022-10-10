use std::io::Read;

use rust_embed_for_web::RustEmbed;

/// Test doc comment
#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Asset;

#[test]
fn get_works() {
    assert!(
        Asset::get("index.html").is_some(),
        "index.html should exist"
    );
    assert!(Asset::get("gg.html").is_none(), "gg.html should not exist");
    assert!(
        Asset::get("images/llama.png").is_some(),
        "llama.png should exist"
    );
}

/// Using Windows-style path separators (`\`) is acceptable
#[test]
fn get_windows_style() {
    assert!(
        Asset::get("images\\llama.png").is_some(),
        "llama.png should be accessible via \"images\\lama.png\""
    );
}

#[test]
fn trait_works_generic() {
    trait_works_generic_helper::<Asset>();
}
fn trait_works_generic_helper<E: rust_embed_for_web::RustEmbed>() {
    assert!(E::get("gg.html").is_none(), "gg.html should not exist");
}

#[test]
fn file_contents_work() {
    let index = Asset::get("index.html").unwrap();
    let index_data = std::str::from_utf8(index.data).unwrap();
    assert!(index_data.starts_with("<!DOCTYPE html>"));
}

#[test]
fn gzipped_file_contents_work() {
    let index = Asset::get("index.html").unwrap();
    let index_data_compressed = index.data_gzip.unwrap();

    let mut gz = flate2::read::GzDecoder::new(index_data_compressed);
    let mut index_data = String::new();

    gz.read_to_string(&mut index_data).unwrap();
    assert!(index_data.starts_with("<!DOCTYPE html>"));
}

#[test]
fn gzipped_data_omitted_for_already_compressed_files() {
    let image = Asset::get("images/flower.jpg").unwrap();

    assert!(image.data_gzip.is_none());
}
