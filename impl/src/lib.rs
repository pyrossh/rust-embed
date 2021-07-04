#![recursion_limit = "1024"]
#![forbid(unsafe_code)]
#[macro_use]
extern crate quote;
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use std::{env, path::Path};
use syn::{Data, DeriveInput, Fields, Lit, Meta, MetaNameValue};

fn embedded(ident: &syn::Ident, folder_path: String, prefix: Option<&str>) -> TokenStream2 {
  extern crate rust_embed_utils;

  let mut match_values = Vec::<TokenStream2>::new();
  let mut list_values = Vec::<String>::new();

  for rust_embed_utils::FileEntry { rel_path, full_canonical_path } in rust_embed_utils::get_files(folder_path) {
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

fn dynamic(ident: &syn::Ident, folder_path: String, prefix: Option<&str>) -> TokenStream2 {
  let (handle_prefix, map_iter) = if let Some(prefix) = prefix {
    (
      quote! { let file_path = file_path.strip_prefix(#prefix)?; },
      quote! { std::borrow::Cow::Owned(format!("{}{}", #prefix, e.rel_path)) },
    )
  } else {
    (TokenStream2::new(), quote! { std::borrow::Cow::from(e.rel_path) })
  };

  quote! {
      #[cfg(debug_assertions)]
      impl #ident {
          pub fn get(file_path: &str) -> Option<rust_embed::EmbeddedFile> {
              #handle_prefix

              let file_path = std::path::Path::new(#folder_path).join(file_path.replace("\\", "/"));
              rust_embed::utils::read_file_from_fs(&file_path).ok()
          }

          pub fn iter() -> impl Iterator<Item = std::borrow::Cow<'static, str>> {
              use std::path::Path;
              rust_embed::utils::get_files(String::from(#folder_path))
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

fn generate_assets(ident: &syn::Ident, folder_path: String, prefix: Option<String>) -> TokenStream2 {
  let embedded_impl = embedded(ident, folder_path.clone(), prefix.as_deref());
  if cfg!(feature = "debug-embed") {
    return embedded_impl;
  }

  let dynamic_impl = dynamic(ident, folder_path, prefix.as_deref());

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

/// Find a `name = "value"` attribute from the derive input
fn find_attribute_value(ast: &syn::DeriveInput, attr_name: &str) -> Option<String> {
  ast
    .attrs
    .iter()
    .find(|value| value.path.is_ident(attr_name))
    .and_then(|attr| attr.parse_meta().ok())
    .and_then(|meta| match meta {
      Meta::NameValue(MetaNameValue { lit: Lit::Str(val), .. }) => Some(val.value()),
      _ => None,
    })
}

fn impl_rust_embed(ast: &syn::DeriveInput) -> TokenStream2 {
  match ast.data {
    Data::Struct(ref data) => match data.fields {
      Fields::Unit => {}
      _ => panic!("RustEmbed can only be derived for unit structs"),
    },
    _ => panic!("RustEmbed can only be derived for unit structs"),
  };

  let folder_path = find_attribute_value(ast, "folder").expect("#[derive(RustEmbed)] should contain one attribute like this #[folder = \"examples/public/\"]");
  let prefix = find_attribute_value(ast, "prefix");

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

  generate_assets(&ast.ident, folder_path, prefix)
}

#[proc_macro_derive(RustEmbed, attributes(folder, prefix))]
pub fn derive_input_object(input: TokenStream) -> TokenStream {
  let ast: DeriveInput = syn::parse(input).unwrap();
  let gen = impl_rust_embed(&ast);
  gen.into()
}
