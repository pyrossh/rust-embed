#[cfg(feature = "compression")]
#[cfg_attr(feature = "compression", doc(hidden))]
pub use include_flate::flate;

#[allow(unused_imports)]
#[macro_use]
extern crate rust_embed_impl;
pub use rust_embed_impl::*;

#[doc(hidden)]
#[cfg(all(debug_assertions, not(feature = "debug-embed")))]
pub extern crate rust_embed_utils as utils;

/// A directory of binary assets.
///
/// They should be embedded into the executable for release builds,
/// but can be read from the filesystem for debug builds.
///
/// This trait is meant to be derived like so:
/// ```
/// use rust_embed::RustEmbed;
///
/// #[derive(RustEmbed)]
/// #[folder = "examples/public/"]
/// struct Asset;
///
/// fn main() {}
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
  fn get(file_path: &str) -> Option<std::borrow::Cow<'static, [u8]>>;

  /// Iterates the files in this assets folder.
  ///
  /// If the feature `debug-embed` is enabled or the binary is compiled in
  /// release mode, a static array to the list of relative paths to the files
  /// is used.
  ///
  /// Otherwise, the files are listed from the file system on each call.
  fn iter() -> Filenames;
}

/// An iterator type over filenames.
///
/// This enum exists for optimization purposes, to avoid boxing the iterator in
/// some cases. Do not try and match on it, as different variants will exist
/// depending on the compilation context.
pub enum Filenames {
  /// Release builds use a nameable iterator type, which can be stack-allocated.
  #[cfg(any(not(debug_assertions), feature = "debug-embed"))]
  Embedded(std::slice::Iter<'static, &'static str>),

  /// The debug iterator type is currently unnamable and still needs to be
  /// boxed.
  #[cfg(all(debug_assertions, not(feature = "debug-embed")))]
  Dynamic(Box<dyn Iterator<Item = std::borrow::Cow<'static, str>>>),
}

impl Iterator for Filenames {
  type Item = std::borrow::Cow<'static, str>;
  fn next(&mut self) -> Option<Self::Item> {
    match self {
      #[cfg(any(not(debug_assertions), feature = "debug-embed"))]
      Filenames::Embedded(names) => names.next().map(|x| std::borrow::Cow::from(*x)),

      #[cfg(all(debug_assertions, not(feature = "debug-embed")))]
      Filenames::Dynamic(boxed) => boxed.next(),
    }
  }
}
