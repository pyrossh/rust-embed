#![recursion_limit = "1024"]
#[macro_use]
extern crate log;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

extern crate walkdir;

use proc_macro::TokenStream;
use syn::*;
use quote::Tokens;

#[cfg(debug_assertions)]
fn generate_assets(ident: &syn::Ident, folder_path: String) -> quote::Tokens {
  quote!{
      use std::fs::File;
      use std::io::Read;
      use std::path::Path;

      impl #ident {
          pub fn get(file_path: &str) -> Option<Vec<u8>> {
              let folder_path = #folder_path;
              let name = &format!("{}{}", folder_path, file_path);
              let path = &Path::new(name);
              let key = String::from(path.to_str().expect("Path does not have a string representation"));
              println!("file: {}", key);
              let mut file = match File::open(path) {
                  Ok(mut file) => file,
                  Err(e) => {
                      eprintln!("file: {} {}", key, e);
                      return None
                  }
              };
              let mut data: Vec<u8> = Vec::new();
              match file.read_to_end(&mut data) {
                  Ok(_) => Some(data),
                  Err(e) =>  {
                      eprintln!("file: {} {}", key, e);
                      return None
                  }
              }
          }
      }
  }
}

#[cfg(not(debug_assertions))]
fn generate_assets(ident: &syn::Ident, folder_path: String) -> quote::Tokens {
  use std::fs::File;
  use std::io::Read;
  use std::path::Path;
  use walkdir::WalkDir;
  let mut values = Vec::<Tokens>::new();
  for entry in WalkDir::new(folder_path.clone())
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.file_type().is_file())
  {
    println!("   \x1b[92mCompiling\x1b[0m {}", entry.path().display());
    let base = &folder_path.clone();
    let key = String::from(
      entry
        .path()
        .to_str()
        .expect("Path does not have a string representation"),
    ).replace(base, "");
    let mut file = File::open(&Path::new(&entry.path())).unwrap_or_else(|e| {
      panic!("could not open file -> {} {}", key, e);
    });
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data).unwrap_or_else(|e| {
      panic!("could not read file -> {} {}", key, e);
    });
    let value = quote!{
      #key => Some(vec!#data),
    };
    values.push(value);
  }
  quote!{
      impl #ident {
          pub fn get(file_path: &str) -> Option<Vec<u8>> {
              match file_path {
                  #(#values)*
                  _ => None,
              }
          }
      }
  }
}

fn help() {
  panic!("#[derive(RustEmbed)] should contain one attribute like this #[folder(\"examples/public/\")]");
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
  let items = match value {
    &MetaItem::List(ref attr_name, ref items) => {
      if attr_name == "folder" {
        items
      } else {
        panic!("#[derive(RustEmbed)] attribute name must be folder");
      }
    }
    _ => {
      panic!("#[derive(RustEmbed)] attribute name must be folder");
    }
  };
  let item = &items[0];
  let lit = match item {
    &NestedMetaItem::Literal(ref l) => l,
    _ => {
      panic!("Hello");
    }
  };
  let folder_path = match lit {
    &Lit::Str(ref val, _) => val.clone(),
    _ => {
      panic!("#[derive(RustEmbed)] attribute value must be a string literal");
    }
  };
  info!("folder: {}", folder_path);
  generate_assets(ident, folder_path)
}

#[proc_macro_derive(RustEmbed, attributes(folder))]
pub fn derive_input_object(input: TokenStream) -> TokenStream {
  let s = input.to_string();
  let ast = syn::parse_derive_input(&s).unwrap();
  let gen = impl_rust_embed(&ast);
  gen.parse().unwrap()
}
