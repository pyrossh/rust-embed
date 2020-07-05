#![recursion_limit = "1024"]
#[macro_use]
extern crate quote;
extern crate proc_macro;

use proc_macro::TokenStream;
use std::{env, path::Path};
use syn::{export::TokenStream2, Data, DeriveInput, Fields, Lit, Meta};

#[allow(unused)]
fn path_to_str<P: AsRef<Path>>(p: P) -> String {
  p.as_ref().to_str().expect("Path does not have a string representation").replace("\\", "/")
}

#[cfg(all(debug_assertions, not(feature = "debug-embed")))]
fn generate_assets(ident: &syn::Ident, folder_path: String) -> TokenStream2 {
  quote! {
      impl #ident {
          fn __rustembed_get(file_path: &std::path::Path) -> Option<std::borrow::Cow<'static, [u8]>> {
              use std::fs;
              use std::path::Path;

              let file_path = Path::new(#folder_path).join(file_path);
              match fs::read(file_path) {
                  Ok(contents) => Some(std::borrow::Cow::from(contents)),
                  Err(_e) =>  {
                      return None
                  }
              }
          }
      }

      impl rust_embed::RustEmbed for #ident {
          fn get<P: AsRef<std::path::Path>>(file_path: P) -> Option<std::borrow::Cow<'static, [u8]>> {
              Self::__rustembed_get(file_path.as_ref())
          }

          fn iter() -> rust_embed::Filenames {
              use std::path::Path;

              rust_embed::Filenames::Dynamic(Box::new(
                  rust_embed::utils::get_files(String::from(#folder_path))
                      .map(|e| e.rel_path)
              ))
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
    let rel_path = path_to_str(rel_path);
    let full_canonical_path = path_to_str(full_canonical_path);

    match_values.push(embed_file(&rel_path, &full_canonical_path));
    list_values.push(rel_path);
  }

  let array_len = list_values.len();

  quote! {
      impl #ident {
          fn __rustembed_get(file_path: &std::path::Path) -> Option<std::borrow::Cow<'static, [u8]>> {
              let file_path = file_path.to_str()?.replace("\\", "/");

              match file_path.as_str() {
                  #(#match_values)*
                  _ => None,
              }
          }
      }

      impl rust_embed::RustEmbed for #ident {
          fn get<P: AsRef<std::path::Path>>(file_path: P) -> Option<std::borrow::Cow<'static, [u8]>> {
              Self::__rustembed_get(file_path.as_ref())
          }

          fn iter() -> rust_embed::Filenames {
              const items: [&str; #array_len] = [#(#list_values),*];
              rust_embed::Filenames::Embedded(items.iter())
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

  generate_assets(&ast.ident, folder_path)
}

#[proc_macro_derive(RustEmbed, attributes(folder))]
pub fn derive_input_object(input: TokenStream) -> TokenStream {
  let ast: DeriveInput = syn::parse(input).unwrap();
  let gen = impl_rust_embed(&ast);
  gen.into()
}
