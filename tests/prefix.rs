use rust_embed_for_web::RustEmbed;

#[derive(RustEmbed)]
#[folder = "examples/public/"]
#[prefix = "prefix/"]
struct Asset;

#[test]
fn get_with_prefix() {
    assert!(Asset::get("prefix/index.html").is_some());
}

#[test]
fn get_without_prefix() {
    assert!(Asset::get("index.html").is_none());
}
