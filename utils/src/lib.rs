#![forbid(unsafe_code)]

use chrono::TimeZone;
use flate2::write::GzEncoder;
use flate2::Compression;
use sha2::digest::generic_array::GenericArray;
use sha2::Digest;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;
use std::{fs, io};

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

pub fn get_files<'patterns>(
    folder_path: String,
    includes: &'patterns [&str],
    excludes: &'patterns [&str],
) -> impl Iterator<Item = FileEntry> + 'patterns {
    walkdir::WalkDir::new(&folder_path)
        .follow_links(true)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(move |e| {
            let rel_path = path_to_str(e.path().strip_prefix(&folder_path).unwrap());
            let full_canonical_path =
                path_to_str(std::fs::canonicalize(e.path()).expect("Could not get canonical path"));

            let rel_path = if std::path::MAIN_SEPARATOR == '\\' {
                rel_path.replace('\\', "/")
            } else {
                rel_path
            };

            if is_path_included(&rel_path, includes, excludes) {
                Some(FileEntry {
                    rel_path,
                    full_canonical_path,
                })
            } else {
                None
            }
        })
}

/// A file embedded into the binary
pub struct EmbeddedFile {
    pub data: &'static [u8],
    pub data_gzip: Option<&'static [u8]>,
    pub metadata: Metadata,
}

/// A file, read from the file system
pub struct ReadFile {
    pub data: Vec<u8>,
    pub data_gzip: Vec<u8>,
    pub metadata: ReadMetadata,
}

/// Metadata about an embedded file
pub struct Metadata {
    pub hash: &'static str,
    pub etag: &'static str,
    pub last_modified: Option<&'static str>,
    pub last_modified_timestamp: Option<i64>,
    pub mime_type: Option<&'static str>,
}

/// Metadata about a file read from the filesystem
pub struct ReadMetadata {
    pub hash: String,
    pub etag: String,
    pub last_modified: Option<String>,
    pub last_modified_timestamp: Option<i64>,
    pub mime_type: Option<String>,
}

pub fn read_file_from_fs(file_path: &Path) -> io::Result<ReadFile> {
    let data = fs::read(file_path)?;

    // During debugging, use no compression to avoid causing slowdowns. For
    // production, we'll go with default compression: it's usually almost as good
    // as best compression but significantly faster.
    let mut encoder = GzEncoder::new(
        Vec::new(),
        if cfg!(debug_assertions) {
            Compression::fast()
        } else {
            Compression::default()
        },
    );
    encoder.write_all(&data).unwrap();
    let data_gzip = encoder.finish().unwrap();

    let mut hasher = sha2::Sha256::new();
    hasher.update(&data);
    let mut hash_bytes = GenericArray::default();
    hasher.finalize_into(&mut hash_bytes);
    let hash = base85::encode(&hash_bytes[..]);

    let last_modified_timestamp = fs::metadata(file_path)?.modified().ok();
    let last_modified = last_modified_timestamp
        .and_then(|value| {
            value
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .map(|value| value.as_secs() as i64)
        })
        .or_else(|| {
            last_modified_timestamp
                .and_then(|value| SystemTime::UNIX_EPOCH.duration_since(value).ok())
                .map(|value| (-1) * (value.as_secs() as i64))
        })
        .map(|timestamp| chrono::Utc.timestamp(timestamp, 0));

    let mime_type = new_mime_guess::from_path(file_path)
        .first()
        .map(|mime| mime.to_string());

    Ok(ReadFile {
        data,
        data_gzip,
        metadata: ReadMetadata {
            etag: format!("\"{hash}\""),
            hash,
            last_modified: last_modified.map(|v| v.to_rfc2822()),
            last_modified_timestamp: last_modified.map(|v| v.timestamp()),
            mime_type,
        },
    })
}

fn path_to_str<P: AsRef<std::path::Path>>(p: P) -> String {
    p.as_ref()
        .to_str()
        .expect("Path does not have a string representation")
        .to_owned()
}
