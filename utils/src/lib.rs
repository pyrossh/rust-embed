#![forbid(unsafe_code)]

use sha2::Digest;
use std::borrow::Cow;
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
  pub metadata: Metadata,
}

/// Metadata about an embedded file
pub struct Metadata {
  hash: [u8; 32],
  last_modified: Option<u64>,
}

impl Metadata {
  #[doc(hidden)]
  pub fn __rust_embed_new(hash: [u8; 32], last_modified: Option<u64>) -> Self {
    Self { hash, last_modified }
  }

  /// The SHA256 hash of the file
  pub fn sha256_hash(&self) -> [u8; 32] {
    self.hash
  }

  /// The last modified date in seconds since the UNIX epoch. If the underlying
  /// platform/file-system does not support this, None is returned.
  pub fn last_modified(&self) -> Option<u64> {
    self.last_modified
  }
}

pub fn read_file_from_fs(file_path: &Path) -> io::Result<EmbeddedFile> {
  let data = fs::read(file_path)?;
  let data = Cow::from(data);

  let mut hasher = sha2::Sha256::new();
  hasher.update(&data);
  let hash: [u8; 32] = hasher.finalize().into();

  let last_modified = fs::metadata(file_path)?.modified().ok().map(|last_modified| {
    last_modified
      .duration_since(SystemTime::UNIX_EPOCH)
      .expect("Time before the UNIX epoch is unsupported")
      .as_secs()
  });

  Ok(EmbeddedFile {
    data,
    metadata: Metadata { hash, last_modified },
  })
}

fn path_to_str<P: AsRef<std::path::Path>>(p: P) -> String {
  p.as_ref().to_str().expect("Path does not have a string representation").to_owned()
}
