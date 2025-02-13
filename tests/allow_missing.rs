use rust_embed::Embed;

#[derive(Embed)]
#[folder = "examples/missing/"]
#[allow_missing = true]
struct Asset;

#[test]
fn missing_is_empty() {
  assert_eq!(Asset::iter().count(), 0);
}
