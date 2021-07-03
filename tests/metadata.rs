use rust_embed::{EmbeddedFile, RustEmbed};
use sha2::Digest;

#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Asset;

#[test]
fn hash_is_accurate() {
  let index_file: EmbeddedFile = Asset::get("index.html").expect("index.html exists");
  let mut hasher = sha2::Sha256::new();
  hasher.update(index_file.data);
  let expected_hash: [u8; 32] = hasher.finalize().into();

  assert_eq!(index_file.metadata.sha256_hash(), expected_hash);
}

#[test]
fn last_modified_is_accurate() {
  let index_file: EmbeddedFile = Asset::get("index.html").expect("index.html exists");
  let expected_datetime_utc = 1527818165;

  assert_eq!(index_file.metadata.last_modified(), Some(expected_datetime_utc));
}
