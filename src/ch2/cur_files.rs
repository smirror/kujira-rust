use std::fs;

fn main() {
    let files = fs::read_dir(".").expect("not exist path");
    for entry in files {
        let entry = entry.unwrap();
        let path = entry.path();
        let fname = path.to_str().unwrap_or("unexpect file's name");
        println!("{}", fname);
    }
}
