#[macro_use]
extern crate log;
extern crate walkdir;

#[cfg(debug_assertions)]
pub fn generate_assets(parent_path: String) -> Box<Fn(String) -> Option<Vec<u8>>> {
    use std::fs::File;
    use std::path::Path;
    use std::io::Read;
    info!("loading folder -> {}", parent_path);
    Box::new(move |file_path| {
        let name = &format!("{}{}", parent_path, file_path);
        let path = &Path::new(name);
        let key = String::from(path.to_str().expect("Path does not have a string representation"));
        info!("asset from file -> {}", key);
        let mut file = match File::open(path) {
            Ok(mut file) => file,
            Err(e) => {
                error!("could not open file -> {} {}", key, e);
                return None
            }
        };
        let mut data: Vec<u8> = Vec::new();
        match file.read_to_end(&mut data) {
            Ok(_) => Some(data),
            Err(e) =>  {
                error!("could not open file -> {} {}", key, e);
                return None
            }
        }
    })
}

#[cfg(not(debug_assertions))]
pub fn generate_assets<'a>(parent_path: String) -> Box<Fn(String) -> Option<Vec<u8>>> {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    use walkdir::WalkDir;
    use std::collections::HashMap;

    info!("loading folder -> {}", parent_path);
    let mut map = HashMap::new();
    for entry in WalkDir::new(parent_path.clone()).into_iter().filter_map(|e| e.ok()).filter(|e| e.file_type().is_file()) {
        info!("asset from file -> {}", entry.path().display());
        let base = &parent_path.clone();
        let key = String::from(entry.path().to_str().expect("Path does not have a string representation")).replace(base, "");
        let mut file = File::open(&Path::new(&entry.path())).unwrap_or_else(|e| {
            panic!("could not open file -> {} {}", key, e);
        });
        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data).unwrap_or_else(|e| {
            panic!("could not read file -> {} {}", key, e);
        });
        map.insert(key, data);
    }
    Box::new(move |file_path| {
        match map.get(&file_path) {
            Some(s) => Some(s.to_vec()),
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
