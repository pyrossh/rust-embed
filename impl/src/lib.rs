#![recursion_limit = "1024"]
#![forbid(unsafe_code)]
#[macro_use]
extern crate quote;
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use std::{env, path::Path};
use syn::{Data, DeriveInput, Fields, Lit, Meta, MetaNameValue};

fn embedded(ident: &syn::Ident, folder_path: String, prefix: Option<&str>, includes: &[String], excludes: &[String]) -> TokenStream2 {
  extern crate rust_embed_utils;

  let mut match_values = Vec::<TokenStream2>::new();
  let mut list_values = Vec::<String>::new();

  let includes: Vec<&str> = includes.iter().map(AsRef::as_ref).collect();
  let excludes: Vec<&str> = excludes.iter().map(AsRef::as_ref).collect();
  for rust_embed_utils::FileEntry { rel_path, full_canonical_path } in rust_embed_utils::get_files(folder_path, &includes, &excludes) {
    match_values.push(embed_file(&rel_path, &full_canonical_path));

    list_values.push(if let Some(prefix) = prefix {
      format!("{}{}", prefix, rel_path)
    } else {
      rel_path
    });
  }

  let array_len = list_values.len();

  // If debug-embed is on, unconditionally include the code below. Otherwise,
  // make it conditional on cfg(not(debug_assertions)).
  let not_debug_attr = if cfg!(feature = "debug-embed") {
    quote! {}
  } else {
    quote! { #[cfg(not(debug_assertions))]}
  };

  let handle_prefix = if let Some(prefix) = prefix {
    quote! {
      let file_path = file_path.strip_prefix(#prefix)?;
    }
  } else {
    TokenStream2::new()
  };

  quote! {
      #not_debug_attr
      impl #ident {
          /// Get an embedded file and its metadata.
          pub fn get(file_path: &str) -> Option<rust_embed::EmbeddedFile> {
            #handle_prefix
            match file_path.replace("\\", "/").as_str() {
                #(#match_values)*
                _ => None,
            }
          }

          fn names() -> std::slice::Iter<'static, &'static str> {
              const items: [&str; #array_len] = [#(#list_values),*];
              items.iter()
          }

          /// Iterates over the file paths in the folder.
          pub fn iter() -> impl Iterator<Item = std::borrow::Cow<'static, str>> {
              Self::names().map(|x| std::borrow::Cow::from(*x))
          }
      }

      #not_debug_attr
      impl rust_embed::RustEmbed for #ident {
        fn get(file_path: &str) -> Option<rust_embed::EmbeddedFile> {
          #ident::get(file_path)
        }
        fn iter() -> rust_embed::Filenames {
          rust_embed::Filenames::Embedded(#ident::names())
        }
      }
  }
}

fn dynamic(ident: &syn::Ident, folder_path: String, prefix: Option<&str>, includes: &[String], excludes: &[String]) -> TokenStream2 {
  let (handle_prefix, map_iter) = if let Some(prefix) = prefix {
    (
      quote! { let file_path = file_path.strip_prefix(#prefix)?; },
      quote! { std::borrow::Cow::Owned(format!("{}{}", #prefix, e.rel_path)) },
    )
  } else {
    (TokenStream2::new(), quote! { std::borrow::Cow::from(e.rel_path) })
  };

  let declare_includes = quote! {
    const includes: &[&str] = &[#(#includes),*];
  };

  let declare_excludes = quote! {
    const excludes: &[&str] = &[#(#excludes),*];
  };

  let canonical_folder_path = Path::new(&folder_path).canonicalize().expect("folder path must resolve to an absolute path");
  let canonical_folder_path = canonical_folder_path.to_str().expect("absolute folder path must be valid unicode");

  quote! {
      #[cfg(debug_assertions)]
      impl #ident {
          /// Get an embedded file and its metadata.
          pub fn get(file_path: &str) -> Option<rust_embed::EmbeddedFile> {
              #handle_prefix

              #declare_includes
              #declare_excludes

              let rel_file_path = file_path.replace("\\", "/");
              let file_path = std::path::Path::new(#folder_path).join(&rel_file_path);

              // Make sure the path requested does not escape the folder path
              let canonical_file_path = file_path.canonicalize().ok()?;
              if !canonical_file_path.starts_with(#canonical_folder_path) {
                  // Tried to request a path that is not in the embedded folder
                  return None;
              }

              if rust_embed::utils::is_path_included(&rel_file_path, includes, excludes) {
                rust_embed::utils::read_file_from_fs(&canonical_file_path).ok()
              } else {
                None
              }
          }

          /// Iterates over the file paths in the folder.
          pub fn iter() -> impl Iterator<Item = std::borrow::Cow<'static, str>> {
              use std::path::Path;

              #declare_includes
              #declare_excludes

              rust_embed::utils::get_files(String::from(#folder_path), includes, excludes)
                  .map(|e| #map_iter)
          }
      }

      #[cfg(debug_assertions)]
      impl rust_embed::RustEmbed for #ident {
        fn get(file_path: &str) -> Option<rust_embed::EmbeddedFile> {
          #ident::get(file_path)
        }
        fn iter() -> rust_embed::Filenames {
          // the return type of iter() is unnamable, so we have to box it
          rust_embed::Filenames::Dynamic(Box::new(#ident::iter()))
        }
      }
  }
}

fn generate_assets(ident: &syn::Ident, folder_path: String, prefix: Option<String>, includes: Vec<String>, excludes: Vec<String>) -> TokenStream2 {
  let embedded_impl = embedded(ident, folder_path.clone(), prefix.as_deref(), &includes, &excludes);
  if cfg!(feature = "debug-embed") {
    return embedded_impl;
  }

  let dynamic_impl = dynamic(ident, folder_path, prefix.as_deref(), &includes, &excludes);

  quote! {
      #embedded_impl
      #dynamic_impl
  }
}

fn embed_file(rel_path: &str, full_canonical_path: &str) -> TokenStream2 {
  let file = rust_embed_utils::read_file_from_fs(Path::new(full_canonical_path)).expect("File should be readable");
  let hash = file.metadata.sha256_hash();
  let last_modified = match file.metadata.last_modified() {
    Some(last_modified) => quote! { Some(#last_modified) },
    None => quote! { None },
  };

  let embedding_code = if cfg!(feature = "compression") {
    quote! {
      rust_embed::flate!(static FILE: [u8] from #full_canonical_path);
      let bytes = &FILE[..];
    }
  } else {
    quote! {
      let bytes = &include_bytes!(#full_canonical_path)[..];
    }
  };

  quote! {
      #rel_path => {
          #embedding_code

          Some(rust_embed::EmbeddedFile {
              data: std::borrow::Cow::from(bytes),
              metadata: rust_embed::Metadata::__rust_embed_new([#(#hash),*], #last_modified)
          })
      }
  }
}

/// Find all pairs of the `name = "value"` attribute from the derive input
fn find_attribute_values(ast: &syn::DeriveInput, attr_name: &str) -> Vec<String> {
  ast
    .attrs
    .iter()
    .filter(|value| value.path.is_ident(attr_name))
    .filter_map(|attr| attr.parse_meta().ok())
    .filter_map(|meta| match meta {
      Meta::NameValue(MetaNameValue { lit: Lit::Str(val), .. }) => Some(val.value()),
      _ => None,
    })
    .collect()
}

fn impl_rust_embed(ast: &syn::DeriveInput) -> TokenStream2 {
  match ast.data {
    Data::Struct(ref data) => match data.fields {
      Fields::Unit => {}
      _ => panic!("RustEmbed can only be derived for unit structs"),
    },
    _ => panic!("RustEmbed can only be derived for unit structs"),
  };

  let mut folder_paths = find_attribute_values(ast, "folder");
  if folder_paths.len() != 1 {
    panic!("#[derive(RustEmbed)] must contain one attribute like this #[folder = \"examples/public/\"]");
  }
  let folder_path = folder_paths.remove(0);

  let prefix = find_attribute_values(ast, "prefix").into_iter().next();
  let includes = find_attribute_values(ast, "include");
  let excludes = find_attribute_values(ast, "exclude");

  #[cfg(feature = "interpolate-folder-path")]
  let folder_path = shellexpand::full(&folder_path).unwrap().to_string();

  // Base relative paths on the Cargo.toml location
  let folder_path = if Path::new(&folder_path).is_relative() {
    Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
      .join(folder_path)
      .to_str()
      .unwrap()
      .to_owned()
  } else {
    folder_path
  };

  if !Path::new(&folder_path).exists() {
    let mut message = format!(
      "#[derive(RustEmbed)] folder '{}' does not exist. cwd: '{}'",
      folder_path,
      std::env::current_dir().unwrap().to_str().unwrap()
    );

    // Add a message about the interpolate-folder-path feature if the path may
    // include a variable
    if folder_path.contains('$') && cfg!(not(feature = "interpolate-folder-path")) {
      message += "\nA variable has been detected. RustEmbed can expand variables \
                  when the `interpolate-folder-path` feature is enabled.";
    }

    panic!("{}", message);
  };

  generate_assets(&ast.ident, folder_path, prefix, includes, excludes)
}

#[proc_macro_derive(RustEmbed, attributes(folder, prefix, include, exclude))]
pub fn derive_input_object(input: TokenStream) -> TokenStream {
  let ast: DeriveInput = syn::parse(input).unwrap();
  let gen = impl_rust_embed(&ast);
  gen.into()
}
