#![recursion_limit = "1024"]
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

#[cfg(feature = "interpolate-folder-path")]
extern crate shellexpand;
extern crate walkdir;

use proc_macro::TokenStream;
use quote::Tokens;
use std::path::Path;
use syn::*;


#[cfg(all(debug_assertions, not(feature = "debug-embed")))]
fn generate_assets(ident: &syn::Ident, folder_path: String) -> quote::Tokens {
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
              extern crate rust_embed_utils;
              rust_embed_utils::get_files(String::from(#folder_path)).map(|e| std::borrow::Cow::from(e.rel_path))
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

#[cfg(any(not(debug_assertions), feature = "debug-embed"))]
fn generate_assets(ident: &syn::Ident, folder_path: String) -> quote::Tokens {
  extern crate rust_embed_utils;
  //use rust_embed_utils::{get_files, FileEntry};

  let mut match_values = Vec::<Tokens>::new();
  let mut list_values = Vec::<String>::new();

  for rust_embed_utils::FileEntry { rel_path, full_canonical_path } in rust_embed_utils::get_files(folder_path) {
    match_values.push(quote! {
      #rel_path => {
          let bytes = &include_bytes!(#full_canonical_path)[..];
          Some(std::borrow::Cow::from(bytes))
      },
    });
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

fn impl_rust_embed(ast: &syn::DeriveInput) -> Tokens {
  match ast.body {
    Body::Enum(_) => panic!("RustEmbed cannot be derived for enums"),
    Body::Struct(ref data) => match data {
      &VariantData::Struct(_) => panic!("RustEmbed can only be derived for unit structs"),
      _ => {}
    },
  };

  let attribute = ast.attrs.iter().map(|attr| &attr.value).find(|value| value.name() == "folder");
  let literal_value = match attribute {
    Some(&MetaItem::NameValue(_, ref literal)) => literal,
    _ => panic!("#[derive(RustEmbed)] should contain one attribute like this #[folder = \"examples/public/\"]"),
  };
  let folder_path = match literal_value {
    &Lit::Str(ref val, _) => val.clone(),
    _ => {
      panic!("#[derive(RustEmbed)] attribute value must be a string literal");
    }
  };

  #[cfg(feature = "interpolate-folder-path")]
  let folder_path = shellexpand::full(&folder_path).unwrap().to_string();

  if !Path::new(&folder_path).exists() {
    panic!(
      "#[derive(RustEmbed)] folder '{}' does not exist. cwd: '{}'",
      folder_path,
      std::env::current_dir().unwrap().to_str().unwrap()
    );
  };

  generate_assets(&ast.ident, folder_path)
}

#[proc_macro_derive(RustEmbed, attributes(folder))]
pub fn derive_input_object(input: TokenStream) -> TokenStream {
  let s = input.to_string();
  let ast = syn::parse_derive_input(&s).unwrap();
  let gen = impl_rust_embed(&ast);
  gen.parse().unwrap()
}
