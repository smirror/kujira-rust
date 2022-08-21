use std::{env, path};
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("findfile (path) (keyword)");
        return;
    }

    let target_dir = &args[1];
    let keyword = &args[2];
    let target = path::PathBuf::from(target_dir);
    findfile(&target, keyword);
}

fn findfile(target: &PathBuf, keyword: &String) {
    let files = target.read_dir().expect("not exist path");
    for dir_entry in files {
        let path = dir_entry.unwrap().path();
        if path.is_dir() {
            findfile(&path, keyword);
            continue;
        }
        let fname = path.file_name().unwrap().to_string_lossy();
        if None == fname.find(keyword) {
            continue;
        }
        println!("{}", path.to_string_lossy());
    }
}


