#![allow(unused_must_use)]
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::vec::Vec;
use std::env;

const INVALIDS: &'static [&'static str] = &[".", "/", "-", "::", ">", "<", " "];

pub fn asset_name_by_filepath(path: &Path) -> String {
    let mut strpath = path.to_str().unwrap().to_string();

    for invalid in INVALIDS {
        strpath = strpath.replace(invalid, "_");
    }
    strpath
}

fn recursive_read(list: &mut Vec<u8>, pp: &mut Vec<u8>, buffer: &mut Vec<u8>, filepath: &Path) {
    match fs::read_dir(filepath) {
        Err(why) => panic!("Directory {} {:?}", filepath.display(), why.kind()),
        Ok(paths) => for entry in paths {
            let path = entry.unwrap().path();
            println!("Reading -> {:?}", path.display());
            if fs::metadata(&path).unwrap().is_dir() {
                recursive_read(list, pp, buffer, &path);
            } else {
                let mut file = File::open(&path).unwrap_or_else(|e| {
                    panic!("couldn't open file {}: {}", e, filepath.display());
                });
                let mut text: Vec<u8> = vec![];
                file.read_to_end(&mut text).unwrap_or_else(|e| {
                    panic!("couldn't read file {}: {}", e, filepath.display());
                });;
                let asset_name = asset_name_by_filepath(&path);
                write!(list, "    \"");
                write!(list, "{}", path.display());
                write!(list, "\",\n");
                write!(pp, "    \"");
                write!(pp, "{}", path.display());
                write!(pp, "\"");
                write!(pp, " => Result::Ok(&");
                write!(pp, "{}", asset_name);
                write!(pp, "),\n");
                write!(buffer, "pub static ");
                write!(buffer, "{}", asset_name);
                write!(buffer, ": [u8; ");
                write!(buffer, "{}", &text.len().to_string());
                write!(buffer, "] = ");
                write!(buffer, "{:?}", text);
                write!(buffer, ";\n");
            }
        },
    }
}

fn print_usage() {
    print!("
rust-embed v0.2.0
Generates rust code to embed resource files into your library or executable

  Usage:
    rust-embed input_folder output_file

  where:
    input_folder  string is the path to the folder containing the assets.
    output_file   string is output filename.

  example:
    rust-embed ./src/public ./src/assets.rs
");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        print_usage();
        return;
    }
    let ref input_folder = args[1];
    let ref output_file = args[2];

    // Recursive read section buffers
    let mut output_buffer: Vec<u8> = vec![];
    let mut pp: Vec<u8> = vec![];
    let mut list: Vec<u8> = vec![];

    write!(output_buffer, "#![allow(dead_code)]");
    write!(output_buffer, "#![allow(non_upper_case_globals)]");
    write!(output_buffer, "#![allow(clippy)]\n");
    write!(list, "{}", "\npub fn list() -> Vec<&'static str> {\n  vec![\n");
    write!(pp, "{}", "\npub fn get(name: &str) -> Result<&[u8], &str> {\n  match name {\n");
    recursive_read(&mut list, &mut pp, &mut output_buffer, Path::new(input_folder));
    write!(list, "{}", "]}");
    write!(pp, "    _=> Result::Err(\"File Not Found\")\n");
    write!(pp, "{}", "  }\n}\n");
    let op = Path::new(output_file);
    println!("Writing -> {:?}", pp);
    println!("Writing -> {:?}", list);
    println!("Writing -> {:?}", op.display());
    let mut file = File::create(&op).unwrap_or_else(|e| {
        panic!("couldn't create {} {:?}", op.display(), e)
    });
    file.write_all(&output_buffer).unwrap_or_else(|e| {
        panic!("couldn't write to {} {:?}", op.display(), e);
    });
    file.write_all(&pp).unwrap_or_else(|e| {
        panic!("couldn't write to {} {:?}", op.display(), e);
    });
    file.write_all(&list).unwrap_or_else(|e| {
        panic!("couldn't write to {} {:?}", op.display(), e);
    });
}
