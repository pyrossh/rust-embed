use rust_embed_for_web::{EmbeddedFile, RustEmbed};
use std::{fs, time::SystemTime};

use chrono::TimeZone;

#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Asset;

#[test]
fn hash_is_accurate() {
    let index_file: EmbeddedFile = Asset::get("index.html").expect("index.html exists");

    let hash = index_file.metadata.hash;
    assert_eq!(hash, "l@tew^Cz<vw>3!wg?Q}D1@!!8DX+Hwg&-~7mA~T$");
}

#[test]
fn last_modified_is_accurate() {
    let index_file: EmbeddedFile = Asset::get("index.html").expect("index.html exists");

    let metadata = fs::metadata(format!(
        "{}/examples/public/index.html",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap();
    let expected_datetime_utc = metadata
        .modified()
        .unwrap()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let expected_datetime = chrono::Utc
        .timestamp(expected_datetime_utc as i64, 0)
        .to_rfc2822();

    assert_eq!(
        index_file.metadata.last_modified,
        Some(expected_datetime.as_str())
    );
}
