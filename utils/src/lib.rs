#![forbid(unsafe_code)]

use chrono::TimeZone;
use flate2::write::GzEncoder;
use flate2::Compression;
use sha2::digest::generic_array::GenericArray;
use sha2::Digest;
use std::borrow::Cow;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;
use std::{fs, io};

#[cfg_attr(all(debug_assertions, not(feature = "debug-embed")), allow(unused))]
pub struct FileEntry {
  pub rel_path: String,
  pub full_canonical_path: String,
}

#[cfg(not(feature = "include-exclude"))]
pub fn is_path_included(_path: &str, _includes: &[&str], _excludes: &[&str]) -> bool {
  true
}

#[cfg(feature = "include-exclude")]
pub fn is_path_included(rel_path: &str, includes: &[&str], excludes: &[&str]) -> bool {
  use globset::Glob;

  // ignore path matched by exclusion pattern
  for exclude in excludes {
    let pattern = Glob::new(exclude)
      .unwrap_or_else(|_| panic!("invalid exclude pattern '{}'", exclude))
      .compile_matcher();

    if pattern.is_match(rel_path) {
      return false;
    }
  }

  // accept path if no includes provided
  if includes.is_empty() {
    return true;
  }

  // accept path if matched by inclusion pattern
  for include in includes {
    let pattern = Glob::new(include)
      .unwrap_or_else(|_| panic!("invalid include pattern '{}'", include))
      .compile_matcher();

    if pattern.is_match(rel_path) {
      return true;
    }
  }

  false
}

#[cfg_attr(all(debug_assertions, not(feature = "debug-embed")), allow(unused))]
pub fn get_files<'patterns>(folder_path: String, includes: &'patterns [&str], excludes: &'patterns [&str]) -> impl Iterator<Item = FileEntry> + 'patterns {
  walkdir::WalkDir::new(&folder_path)
    .follow_links(true)
    .sort_by_file_name()
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.file_type().is_file())
    .filter_map(move |e| {
      let rel_path = path_to_str(e.path().strip_prefix(&folder_path).unwrap());
      let full_canonical_path = path_to_str(std::fs::canonicalize(e.path()).expect("Could not get canonical path"));

      let rel_path = if std::path::MAIN_SEPARATOR == '\\' {
        rel_path.replace('\\', "/")
      } else {
        rel_path
      };

      if is_path_included(&rel_path, includes, excludes) {
        Some(FileEntry { rel_path, full_canonical_path })
      } else {
        None
      }
    })
}

/// A file embedded into the binary
pub struct EmbeddedFile {
  pub data: Cow<'static, [u8]>,
  pub data_gzip: Cow<'static, [u8]>,
  pub metadata: Metadata,
}

/// Metadata about an embedded file
pub struct Metadata {
  hash: String,
  etag: String,
  last_modified: Option<String>,
  mime_type: Option<String>,
}

impl Metadata {
  #[doc(hidden)]
  pub fn __rust_embed_new(hash: String, etag: String, last_modified: Option<String>, mime_type: Option<String>) -> Self {
    Self {
      hash,
      etag,
      last_modified,
      mime_type,
    }
  }

  /// The SHA256 hash of the file contents, base64 encoded.
  pub fn sha256_hash(&self) -> &str {
    self.hash.as_str()
  }

  /// The `sha256_hash`, surrounded by quotes. This is the format required in
  /// `ETag` headers.
  pub fn etag(&self) -> &str {
    self.etag.as_str()
  }

  /// The last modified date in the rfc2822 format. This is the format required
  /// in `Last-Modified` headers.
  ///
  /// This may be None on some platforms that don't support last modified
  /// timestamps.
  pub fn last_modified(&self) -> Option<&str> {
    self.last_modified.as_ref().map(String::as_str)
  }

  pub fn mime_type(&self) -> Option<&str> {
    self.mime_type.as_ref().map(String::as_str)
  }
}

pub fn read_file_from_fs(file_path: &Path) -> io::Result<EmbeddedFile> {
  let data = fs::read(file_path)?;
  let data = Cow::from(data);

  // During debugging, use no compression to avoid causing slowdowns. For
  // production, we'll go with default compression: it's usually almost as good
  // as best compression but significantly faster.
  let mut encoder = GzEncoder::new(Vec::new(), if cfg!(debug_assertions) { Compression::none() } else { Compression::default() });
  encoder.write_all(&data).unwrap();
  let data_gzip = encoder.finish().unwrap();
  let data_gzip = Cow::from(data_gzip);

  let mut hasher = sha2::Sha256::new();
  hasher.update(&data);
  let mut hash_bytes = GenericArray::default();
  hasher.finalize_into(&mut hash_bytes);
  let hash = base85::encode(&hash_bytes[..]);

  let last_modified_timestamp = fs::metadata(file_path)?.modified().ok();
  let last_modified = last_modified_timestamp
    .and_then(|value| value.duration_since(SystemTime::UNIX_EPOCH).ok().map(|value| value.as_secs() as i64))
    .or_else(|| {
      last_modified_timestamp
        .and_then(|value| SystemTime::UNIX_EPOCH.duration_since(value).ok())
        .map(|value| (-1) * (value.as_secs() as i64))
    })
    .map(|timestamp| chrono::Utc.timestamp(timestamp, 0).to_rfc2822());

  let mime_type = new_mime_guess::from_path(file_path).first().map(|mime| mime.to_string());

  Ok(EmbeddedFile {
    data,
    data_gzip,
    metadata: Metadata {
      etag: format!("\"{hash}\""),
      hash,
      last_modified,
      mime_type,
    },
  })
}

fn path_to_str<P: AsRef<std::path::Path>>(p: P) -> String {
  p.as_ref().to_str().expect("Path does not have a string representation").to_owned()
}
