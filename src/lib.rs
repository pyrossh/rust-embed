#![forbid(unsafe_code)]

#[allow(unused_imports)]
#[macro_use]
extern crate rust_embed_for_web_impl;
pub use rust_embed_for_web_impl::*;

pub use rust_embed_for_web_utils::{EmbeddedFile, Metadata};

#[doc(hidden)]
pub extern crate rust_embed_for_web_utils as utils;

/// A directory of binary assets.
///
/// The files in the specified folder will be embedded into the executable in
/// release builds.
///
/// This trait is meant to be derived like so:
/// ```
/// use rust_embed_for_web::RustEmbed;
///
/// #[derive(RustEmbed)]
/// #[folder = "examples/public/"]
/// struct Asset;
///
/// fn main() {}
/// ```
pub trait RustEmbed {
    /// Get an embedded file and its metadata.
    fn get(file_path: &str) -> Option<EmbeddedFile>;
}
