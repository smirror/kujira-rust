use std::fs;

fn main() {
    let afile = "./fizzbuzz_py.txt";
    let bfile = "./fizzbuzz_rs.txt"

    let astr = fs::read_to_string(afile).unwrap().trim();
    let bstr = fs::read_to_string(bfile).unwrap().trim();

    if astr == bstr {
        println!("OK")
    } else {
        println!("NG")
    }
}
