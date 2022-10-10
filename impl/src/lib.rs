#![recursion_limit = "1024"]
#![forbid(unsafe_code)]
#[macro_use]
extern crate quote;
extern crate proc_macro;

/// Only include the gzipped version if it is at least this much smaller than
/// the uncompressed version.
const GZIP_INCLUDE_THRESHOLD: f64 = 0.95;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use std::{env, path::Path};
use syn::{Data, DeriveInput, Fields, Lit, Meta, MetaNameValue};

fn embedded(
    ident: &syn::Ident,
    folder_path: String,
    prefix: Option<&str>,
    includes: &[String],
    excludes: &[String],
) -> TokenStream2 {
    extern crate rust_embed_for_web_utils;

    let mut match_values = Vec::<TokenStream2>::new();
    let mut list_values = Vec::<String>::new();

    let includes: Vec<&str> = includes.iter().map(AsRef::as_ref).collect();
    let excludes: Vec<&str> = excludes.iter().map(AsRef::as_ref).collect();
    for rust_embed_for_web_utils::FileEntry {
        rel_path,
        full_canonical_path,
    } in rust_embed_for_web_utils::get_files(folder_path, &includes, &excludes)
    {
        match_values.push(embed_file(&rel_path, &full_canonical_path));

        list_values.push(if let Some(prefix) = prefix {
            format!("{}{}", prefix, rel_path)
        } else {
            rel_path
        });
    }

    let handle_prefix = if let Some(prefix) = prefix {
        quote! {
          let file_path = file_path.strip_prefix(#prefix)?;
        }
    } else {
        TokenStream2::new()
    };

    quote! {
        impl #ident {
            /// Get an embedded file and its metadata.
            pub fn get(file_path: &str) -> Option<rust_embed_for_web::EmbeddedFile> {
              #handle_prefix
              match file_path.replace("\\", "/").as_str() {
                  #(#match_values)*
                  _ => None,
              }
            }
        }

        impl rust_embed_for_web::RustEmbed for #ident {
          fn get(file_path: &str) -> Option<rust_embed_for_web::EmbeddedFile> {
            #ident::get(file_path)
          }
        }
    }
}

fn generate_assets(
    ident: &syn::Ident,
    folder_path: String,
    prefix: Option<String>,
    includes: Vec<String>,
    excludes: Vec<String>,
) -> TokenStream2 {
    let embedded_impl = embedded(
        ident,
        folder_path.clone(),
        prefix.as_deref(),
        &includes,
        &excludes,
    );

    quote! {
        #embedded_impl
    }
}

fn embed_file(rel_path: &str, full_canonical_path: &str) -> TokenStream2 {
    let file = rust_embed_for_web_utils::read_file_from_fs(Path::new(full_canonical_path))
        .expect("File should be readable");
    let hash = file.metadata.sha256_hash();
    let etag = file.metadata.etag();
    let last_modified = match file.metadata.last_modified() {
        Some(last_modified) => quote! { Some(#last_modified) },
        None => quote! { None },
    };
    let mime_type = match file.metadata.mime_type() {
        Some(mime_type) => quote! { Some(#mime_type ) },
        None => quote! { None },
    };

    let data = file.data;
    let data_len = data.len();
    let data_gzip = file.data_gzip;
    let data_gzip_len = data_gzip.len();

    // Sometimes, the gzipped data is barely any smaller than the original data
    // or it may even be larger. This especially happens in files that are way
    // too small, or files in already compressed formats like images and videos.
    let include_data_gzip = data_gzip_len < (data_len as f64 * GZIP_INCLUDE_THRESHOLD) as usize;
    let data_gzip_data_embed = if include_data_gzip {
        quote! {
            static data_gzip: [u8; #data_gzip_len] = [#(#data_gzip),*];
        }
    } else {
        quote! {}
    };
    let data_gzip_value_embed = if include_data_gzip {
        quote! {
            Some(&data_gzip)
        }
    } else {
        quote! {
            None
        }
    };

    let embed_data = quote! {
      static data: [u8; #data_len] = [#(#data),*];
      #data_gzip_data_embed
    };

    quote! {
        #rel_path => {
          #embed_data

            Some(rust_embed_for_web::EmbeddedFile {
                data: &data,
                data_gzip: #data_gzip_value_embed,
                metadata: rust_embed_for_web::Metadata::__rust_embed_for_web_new(#hash, #etag, #last_modified, #mime_type)
            })
        }
    }
}

/// Find all pairs of the `name = "value"` attribute from the derive input
fn find_attribute_values(ast: &syn::DeriveInput, attr_name: &str) -> Vec<String> {
    ast.attrs
        .iter()
        .filter(|value| value.path.is_ident(attr_name))
        .filter_map(|attr| attr.parse_meta().ok())
        .filter_map(|meta| match meta {
            Meta::NameValue(MetaNameValue {
                lit: Lit::Str(val), ..
            }) => Some(val.value()),
            _ => None,
        })
        .collect()
}

fn impl_rust_embed_for_web(ast: &syn::DeriveInput) -> TokenStream2 {
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

    generate_assets(&ast.ident, folder_path, prefix, includes, excludes)
}

#[proc_macro_derive(RustEmbed, attributes(folder, prefix, include, exclude))]
pub fn derive_input_object(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let gen = impl_rust_embed_for_web(&ast);
    gen.into()
}
