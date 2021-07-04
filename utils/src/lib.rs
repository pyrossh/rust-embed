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

#[cfg_attr(all(debug_assertions, not(feature = "debug-embed")), allow(unused))]
pub fn get_files(folder_path: String) -> impl Iterator<Item = FileEntry> {
  walkdir::WalkDir::new(&folder_path)
    .follow_links(true)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.file_type().is_file())
    .map(move |e| {
      let rel_path = path_to_str(e.path().strip_prefix(&folder_path).unwrap());
      let full_canonical_path = path_to_str(std::fs::canonicalize(e.path()).expect("Could not get canonical path"));

      let rel_path = if std::path::MAIN_SEPARATOR == '\\' {
        rel_path.replace('\\', "/")
      } else {
        rel_path
      };

      FileEntry { rel_path, full_canonical_path }
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
