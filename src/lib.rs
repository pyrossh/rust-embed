#[macro_use]
extern crate log;
extern crate walkdir;

#[cfg(debug_assertions)]
pub fn generate_assets(parent_path: String) -> Box<Fn(String) -> Option<String>> {
    use std::fs::File;
    use std::path::Path;
    use std::io::Read;
    info!("rust-embed: loading folder -> {}", parent_path);
    Box::new(move |file_path| {
        let path = format!("{}{}", parent_path, file_path);
        info!("rust-embed: asset from file -> {}", path);
        let mut file = match File::open(&Path::new(&path)) {
            Ok(mut file) => file,
            Err(e) => {
                println!("rust-embed: could not open file -> {} {}", path, e);
                return None
            }
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => Some(contents),
            Err(e) =>  {
                println!("rust-embed: could not open file -> {} {}", path, e);
                return None
            }
        }
    })
}

#[cfg(not(debug_assertions))]
pub fn generate_assets(parent_path: String) -> Box<Fn(String) -> Option<String>> {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    use walkdir::WalkDir;
    use std::collections::HashMap;

    println!("rust-embed: loading folder -> {}", parent_path);
    let mut map = HashMap::new();
    for entry in WalkDir::new(parent_path).into_iter().filter_map(|e| e.ok()).filter(|e| e.file_type().is_file()) {
        println!("rust-embed: asset from file -> {}", entry.path().display());
        let key = String::from(entry.path().to_str().expect("Path does not have a string representation"));
        let mut file = File::open(&Path::new(&entry.path())).unwrap_or_else(|e| {
            panic!("rust-embed: could not open file -> {} {}", key, e);
        });
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap_or_else(|e| {
            panic!("rust-embed: could not read file -> {} {}", key, e);
        });
        map.insert(key, contents);
    }
    Box::new(move |file_path| {
        match map.get(&file_path) {
            Some(s) => Some(s.to_string()),
            None => None,
        }
    })
}

#[macro_export]
macro_rules! embed {
    ($x:expr) => ( ::generate_assets($x) )
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(debug_assertions)]
    fn dev() {
        let asset = embed!("examples/public".to_owned());
        match asset("/index.html".to_owned()) {
            None => assert!(false, "index.html should exist"),
            _ => assert!(true),
        }
        match asset("/gg.html".to_owned()) {
            Some(_) => assert!(false, "gg.html should not exist"),
            _ => assert!(true),
        }
        match asset("/images/llama.png".to_owned()) {
            None => assert!(false, "llama.png should exist"),
            _ => assert!(true),
        }
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn prod() {
        let asset = embed!("examples/public".to_owned());
        match asset("/index.html".to_owned()) {
            None => assert!(false, "index.html should exist"),
            _ => assert!(true),
        }
        match asset("/gg.html".to_owned()) {
            Some(_) => assert!(false, "gg.html should not exist"),
            _ => assert!(true),
        }
        match asset("/images/llama.png".to_owned()) {
            None => assert!(false, "llama.png should exist"),
            _ => assert!(true),
        }
    }
}
