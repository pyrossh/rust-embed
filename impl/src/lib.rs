#![recursion_limit = "1024"]
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
          pub fn get(file_path: &str) -> Option<std::borrow::Cow<'static, [u8]>> {
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
        fn get(file_path: &str) -> Option<std::borrow::Cow<'static, [u8]>> {
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
          pub fn get(file_path: &str) -> Option<std::borrow::Cow<'static, [u8]>> {
              use std::fs;
              use std::path::Path;

              #handle_prefix

              let file_path = Path::new(#folder_path).join(file_path.replace("\\", "/"));
              match fs::read(file_path) {
                  Ok(contents) => Some(std::borrow::Cow::from(contents)),
                  Err(_e) =>  {
                      return None
                  }
              }
          }

          pub fn iter() -> impl Iterator<Item = std::borrow::Cow<'static, str>> {
              use std::path::Path;
              rust_embed::utils::get_files(String::from(#folder_path))
                  .map(|e| #map_iter)
          }
      }

      #[cfg(debug_assertions)]
      impl rust_embed::RustEmbed for #ident {
        fn get(file_path: &str) -> Option<std::borrow::Cow<'static, [u8]>> {
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

#[cfg(not(feature = "compression"))]
fn embed_file(rel_path: &str, full_canonical_path: &str) -> TokenStream2 {
  quote! {
    #rel_path => {
        let bytes = &include_bytes!(#full_canonical_path)[..];
        Some(std::borrow::Cow::from(bytes))
    },
  }
}

#[cfg(feature = "compression")]
fn embed_file(rel_path: &str, full_canonical_path: &str) -> TokenStream2 {
  quote! {
    #rel_path => {
        rust_embed::flate!(static FILE: [u8] from #full_canonical_path);

        let bytes = &FILE[..];
        Some(std::borrow::Cow::from(bytes))
    },
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

    panic!(message);
  };

  generate_assets(&ast.ident, folder_path, prefix)
}

#[proc_macro_derive(RustEmbed, attributes(folder, prefix))]
pub fn derive_input_object(input: TokenStream) -> TokenStream {
  let ast: DeriveInput = syn::parse(input).unwrap();
  let gen = impl_rust_embed(&ast);
  gen.into()
}
