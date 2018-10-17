#![recursion_limit = "1024"]
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

extern crate walkdir;

use proc_macro::TokenStream;
use quote::Tokens;
use std::path::Path;
use syn::*;

mod utils;

#[cfg(all(debug_assertions, not(feature = "debug-embed")))]
fn generate_assets(ident: &syn::Ident, folder_path: String) -> quote::Tokens {
  quote!{
      impl #ident {
          pub fn get(file_path: &str) -> Option<impl AsRef<[u8]>> {
              use std::fs::File;
              use std::io::Read;
              use std::path::Path;

              let folder_path = #folder_path;
              let name = &format!("{}{}", folder_path, file_path);
              let path = &Path::new(name);
              let mut file = match File::open(path) {
                  Ok(mut file) => file,
                  Err(_e) => {
                      return None
                  }
              };
              let mut data: Vec<u8> = Vec::new();
              match file.read_to_end(&mut data) {
                  Ok(_) => Some(data),
                  Err(_e) =>  {
                      return None
                  }
              }
          }

          pub fn iter() -> impl Iterator<Item = impl AsRef<str>> {
              use rust_embed::utils::get_files;
              get_files(String::from(#folder_path)).map(|e| e.rel_path)
          }
      }
  }
}

#[cfg(any(not(debug_assertions), feature = "debug-embed"))]
fn generate_assets(ident: &syn::Ident, folder_path: String) -> quote::Tokens {
  use utils::{get_files, FileEntry};

  let mut match_values = Vec::<Tokens>::new();
  let mut list_values = Vec::<String>::new();

  for FileEntry { rel_path, full_canonical_path } in get_files(folder_path) {
    match_values.push(quote!{
      #rel_path => Some(&include_bytes!(#full_canonical_path)[..]),
    });
    list_values.push(rel_path);
  }

  let array_len = list_values.len();

  quote!{
      impl #ident {
          pub fn get(file_path: &str) -> Option<impl AsRef<[u8]>> {
              match file_path {
                  #(#match_values)*
                  _ => None,
              }
          }

          pub fn iter() -> impl Iterator<Item = impl AsRef<str>> {
              static items: [&str; #array_len] = [#(#list_values),*];
              items.iter()
          }
      }
  }
}

fn help() {
  panic!("#[derive(RustEmbed)] should contain one attribute like this #[folder = \"examples/public/\"]");
}

fn impl_rust_embed(ast: &syn::DeriveInput) -> Tokens {
  match ast.body {
    Body::Enum(_) => help(),
    Body::Struct(ref data) => match data {
      &VariantData::Struct(_) => help(),
      _ => {}
    },
  };
  let ident = &ast.ident;
  if ast.attrs.len() == 0 || ast.attrs.len() > 1 {
    help();
  }
  let value = &ast.attrs[0].value;
  let literal_value = match value {
    &MetaItem::NameValue(ref attr_name, ref value) => {
      if attr_name == "folder" {
        value
      } else {
        panic!("#[derive(RustEmbed)] attribute name must be folder");
      }
    }
    _ => {
      panic!("#[derive(RustEmbed)] attribute name must be folder");
    }
  };
  let folder_path = match literal_value {
    &Lit::Str(ref val, _) => val.clone(),
    _ => {
      panic!("#[derive(RustEmbed)] attribute value must be a string literal");
    }
  };
  if !Path::new(&folder_path).exists() {
    panic!(
      "#[derive(RustEmbed)] folder '{}' does not exist. cwd: '{}'",
      folder_path,
      std::env::current_dir().unwrap().to_str().unwrap()
    );
  };
  generate_assets(ident, folder_path)
}

#[proc_macro_derive(RustEmbed, attributes(folder))]
pub fn derive_input_object(input: TokenStream) -> TokenStream {
  let s = input.to_string();
  let ast = syn::parse_derive_input(&s).unwrap();
  let gen = impl_rust_embed(&ast);
  gen.parse().unwrap()
}
