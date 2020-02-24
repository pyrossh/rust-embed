#![recursion_limit = "1024"]
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

#[cfg(feature = "interpolate-folder-path")]
extern crate shellexpand;
extern crate walkdir;

use proc_macro::TokenStream;
use std::path::Path;
use syn::{export::TokenStream2, Data, DeriveInput, Fields, Lit, Meta};

#[cfg(all(debug_assertions, not(feature = "debug-embed")))]
fn generate_assets(ident: &syn::Ident, folder_path: String) -> TokenStream2 {
  quote! {
      impl #ident {
          pub fn get(file_path: &str) -> Option<std::borrow::Cow<'static, [u8]>> {
              use std::fs::File;
              use std::io::Read;
              use std::path::Path;

              let file_path = Path::new(#folder_path).join(file_path);
              let mut file = match File::open(file_path) {
                  Ok(mut file) => file,
                  Err(_e) => {
                      return None
                  }
              };
              let mut data: Vec<u8> = Vec::new();
              match file.read_to_end(&mut data) {
                  Ok(_) => Some(std::borrow::Cow::from(data)),
                  Err(_e) =>  {
                      return None
                  }
              }
          }

          pub fn iter() -> impl Iterator<Item = std::borrow::Cow<'static, str>> {
              use std::path::Path;
              rust_embed::utils::get_files(String::from(#folder_path)).map(|e| std::borrow::Cow::from(e.rel_path))
          }
      }
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

#[cfg(all(not(feature = "compression"), any(not(debug_assertions), feature = "debug-embed")))]
fn embed_file(rel_path: &str, full_canonical_path: &str) -> TokenStream2 {
  quote! {
    #rel_path => {
        let bytes = &include_bytes!(#full_canonical_path)[..];
        Some(std::borrow::Cow::from(bytes))
    },
  }
}

#[cfg(all(feature = "compression", any(not(debug_assertions), feature = "debug-embed")))]
fn embed_file(rel_path: &str, full_canonical_path: &str) -> TokenStream2 {
  quote! {
    #rel_path => {
        rust_embed::flate!(static FILE: [u8] from #full_canonical_path);

        let bytes = &FILE[..];
        Some(std::borrow::Cow::from(bytes))
    },
  }
}

#[cfg(any(not(debug_assertions), feature = "debug-embed"))]
fn generate_assets(ident: &syn::Ident, folder_path: String) -> TokenStream2 {
  extern crate rust_embed_utils;

  let mut match_values = Vec::<TokenStream2>::new();
  let mut list_values = Vec::<String>::new();

  for rust_embed_utils::FileEntry { rel_path, full_canonical_path } in rust_embed_utils::get_files(folder_path) {
    match_values.push(embed_file(&rel_path, &full_canonical_path));
    list_values.push(rel_path);
  }

  let array_len = list_values.len();

  quote! {
      impl #ident {
          pub fn get(file_path: &str) -> Option<std::borrow::Cow<'static, [u8]>> {
              match file_path {
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

fn impl_rust_embed(ast: &syn::DeriveInput) -> TokenStream2 {
  match ast.data {
    Data::Struct(ref data) => match data.fields {
      Fields::Unit => {}
      _ => panic!("RustEmbed can only be derived for unit structs"),
    },
    _ => panic!("RustEmbed can only be derived for unit structs"),
  };

  let attribute = ast
    .attrs
    .iter()
    .find(|value| value.path.is_ident("folder"))
    .expect("#[derive(RustEmbed)] should contain one attribute like this #[folder = \"examples/public/\"]");
  let meta = attribute
    .parse_meta()
    .expect("#[derive(RustEmbed)] should contain one attribute like this #[folder = \"examples/public/\"]");
  let literal_value = match meta {
    Meta::NameValue(ref data) => &data.lit,
    _ => panic!("#[derive(RustEmbed)] should contain one attribute like this #[folder = \"examples/public/\"]"),
  };
  let folder_path = match literal_value {
    Lit::Str(ref val) => val.clone().value(),
    _ => {
      panic!("#[derive(RustEmbed)] attribute value must be a string literal");
    }
  };

  #[cfg(feature = "interpolate-folder-path")]
  let folder_path = shellexpand::full(&folder_path).unwrap().to_string();

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

  generate_assets(&ast.ident, folder_path)
}

#[proc_macro_derive(RustEmbed, attributes(folder))]
pub fn derive_input_object(input: TokenStream) -> TokenStream {
  let ast: DeriveInput = syn::parse(input).unwrap();
  let gen = impl_rust_embed(&ast);
  gen.into()
}
