use std::path::PathBuf;

#[cfg_attr(all(debug_assertions, not(feature = "debug-embed")), allow(unused))]
pub struct FileEntry {
  pub rel_path: PathBuf,
  pub full_canonical_path: PathBuf,
}

#[cfg_attr(all(debug_assertions, not(feature = "debug-embed")), allow(unused))]
pub fn get_files(folder_path: String) -> impl Iterator<Item = FileEntry> {
  walkdir::WalkDir::new(&folder_path)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.file_type().is_file())
    .map(move |e| {
      let rel_path = e.path().strip_prefix(&folder_path).unwrap().to_owned();
      let full_canonical_path = std::fs::canonicalize(e.path()).expect("Could not get canonical path");

      FileEntry { rel_path, full_canonical_path }
    })
}
