#[cfg_attr(all(debug_assertions, not(feature = "debug-embed")), allow(unused))]
pub struct FileEntry {
  pub rel_path: String,
  pub full_canonical_path: String,
}

#[cfg_attr(all(debug_assertions, not(feature = "debug-embed")), allow(unused))]
pub fn get_files(folder_path: String) -> impl Iterator<Item = FileEntry> {
  use std;
  use walkdir;
  walkdir::WalkDir::new(&folder_path)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.file_type().is_file())
    .map(move |e| {
      let rel_path = e
        .path()
        .strip_prefix(&folder_path)
        .unwrap()
        .to_str()
        .expect("Path does not have a string representation")
        .to_owned();

      let full_canonical_path = std::fs::canonicalize(e.path())
        .expect("Could not get canonical path")
        .to_str()
        .expect("Path does not have a string representation")
        .to_owned();

      let rel_path = if std::path::MAIN_SEPARATOR == '\\' {
        rel_path.replace('\\', "/")
      } else {
        rel_path
      };

      FileEntry { rel_path, full_canonical_path }
    })
}
