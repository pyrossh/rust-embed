use rust_embed_for_web::RustEmbed;

#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct AllAssets;

#[test]
fn get_works() {
    assert!(
        AllAssets::get("index.html").is_some(),
        "index.html should exist"
    );
    assert!(
        AllAssets::get("gg.html").is_none(),
        "gg.html should not exist"
    );
    assert!(
        AllAssets::get("images/llama.png").is_some(),
        "llama.png should exist"
    );
}

#[derive(RustEmbed)]
#[folder = "examples/public/"]
#[include = "*.html"]
#[include = "images/*"]
struct IncludeSomeAssets;

#[test]
fn including_some_assets_works() {
    assert!(
        IncludeSomeAssets::get("index.html").is_some(),
        "index.html should exist"
    );
    assert!(
        IncludeSomeAssets::get("main.js").is_none(),
        "main.js should not exist"
    );
    assert!(
        IncludeSomeAssets::get("images/llama.png").is_some(),
        "llama.png should exist"
    );
}

#[derive(RustEmbed)]
#[folder = "examples/public/"]
#[exclude = "*.html"]
#[exclude = "images/*"]
struct ExcludeSomeAssets;

#[test]
fn excluding_some_assets_works() {
    assert!(
        ExcludeSomeAssets::get("index.html").is_none(),
        "index.html should not exist"
    );
    assert!(
        ExcludeSomeAssets::get("main.js").is_some(),
        "main.js should exist"
    );
    assert!(
        ExcludeSomeAssets::get("images/llama.png").is_none(),
        "llama.png should not exist"
    );
}

#[derive(RustEmbed)]
#[folder = "examples/public/"]
#[include = "images/*"]
#[exclude = "*.txt"]
struct ExcludePriorityAssets;

#[test]
fn exclude_has_higher_priority() {
    assert!(
        ExcludePriorityAssets::get("images/doc.txt").is_none(),
        "doc.txt should not exist"
    );
    assert!(
        ExcludePriorityAssets::get("images/llama.png").is_some(),
        "llama.png should exist"
    );
}
