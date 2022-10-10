use rust_embed_for_web::RustEmbed;

/// Test doc comment
#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/examples/public/"]
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

#[test]
fn trait_works_generic() {
    trait_works_generic_helper::<Asset>();
}
fn trait_works_generic_helper<E: rust_embed_for_web::RustEmbed>() {
    assert!(E::get("gg.html").is_none(), "gg.html should not exist");
}
