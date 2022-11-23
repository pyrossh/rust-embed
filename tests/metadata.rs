use rust_embed::{EmbeddedFile, RustEmbed};
use sha2::Digest;
use std::{fs, time::SystemTime};

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

  let metadata = fs::metadata(format!("{}/examples/public/index.html", env!("CARGO_MANIFEST_DIR"))).unwrap();
  let expected_datetime_utc = metadata.modified().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

  assert_eq!(index_file.metadata.last_modified(), Some(expected_datetime_utc));
}
