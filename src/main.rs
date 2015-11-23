use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::vec::Vec;
use std::env;

fn recursive_read(buffer: &mut Vec<u8>, filepath: &Path) {
    match fs::read_dir(filepath) {
        Err(why) => panic!("Directory {} {:?}", filepath.display(), why.kind()),
        Ok(paths) => for entry in paths {
            let path = entry.unwrap().path();
            println!("Reading -> {:?}", path.display());
            if fs::metadata(&path).unwrap().is_dir() {
                recursive_read(buffer, &path);
            } else {
                let mut file = File::open(&path).unwrap_or_else(|e| {
                    panic!("couldn't open file {}: {}", e, filepath.display());
                });
                let mut text: Vec<u8> = vec![];
                file.read_to_end(&mut text).unwrap_or_else(|e| {
                    panic!("couldn't read file {}: {}", e, filepath.display());
                });
                write!(buffer, "{}", "pub static ");
                write!(buffer, "{}", path.file_name().unwrap().to_str().unwrap().replace(".", "_").replace("/", "_"));
                write!(buffer, "{}", ": [u8; ");
                write!(buffer, "{}", &text.len().to_string());
                write!(buffer, "{}", "] = ");
                write!(buffer, "{:?}", text);
                write!(buffer, "{}", ";\n");
            }
        },
    }
}

fn print_usage() {
    print!("
rust-embed v0.1.0
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
    let mut output_buffer: Vec<u8> = vec![];
    recursive_read(&mut output_buffer, Path::new(input_folder));
    let op = Path::new(output_file);
    println!("Writing -> {:?}", op.display());
    let mut file = File::create(&op).unwrap_or_else(|e| {
        panic!("couldn't create {} {:?}", op.display(), e)
    });
    file.write_all(&output_buffer).unwrap_or_else(|e| {
        panic!("couldn't write to {} {:?}", op.display(), e);
    });
}
