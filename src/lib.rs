#[cfg(all(debug_assertions, not(feature = "debug-embed")))]
extern crate walkdir;

#[allow(unused_imports)]
#[macro_use]
extern crate rust_embed_impl;
pub use rust_embed_impl::*;

#[doc(hidden)]
#[cfg(all(debug_assertions, not(feature = "debug-embed")))]
pub mod utils;

/// A directory of binary assets.
///
/// They should be embedded into the executable for release builds,
/// but can be read from the filesystem for debug builds.
///
/// This trait is meant to be derived like so:
/// ```
/// #[macro_use]
/// extern crate rust_embed;
/// #[derive(RustEmbed)]
/// #[folder = "examples/public/"]
/// struct Asset;
/// ```

pub trait RustEmbed {
  /// Given a relative path from the assets folder, returns the bytes if found.
  ///
  /// If the feature `debug-embed` is enabled or the binary is compiled in
  /// release mode, the bytes have been embeded in the binary and a
  /// `Cow::Borrowed(&'static [u8])` is returned.
  ///
  /// Otherwise, the bytes are read from the file system on each call and a
  /// `Cow::Owned(Vec<u8>)` is returned.
  fn get(&self, file_path: &str) -> Option<std::borrow::Cow<'static, [u8]>>;

  /// Iterates the files in this assets folder.
  ///
  /// If the feature `debug-embed` is enabled or the binary is compiled in
  /// release mode, a static array to the list of relative paths to the files
  /// is used.
  ///
  /// Otherwise, the files are listed from the file system on each call.
  fn iter(&self) -> Filenames;
}
