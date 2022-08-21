use std::collections::HashMap;

const V_DATA: &str = "c,c,a,a,a,b,c,c,b,b,b,c,b,c,b,a,c,c,b,c,c,c";

fn main() {
    let mut c_map = HashMap::new();
    c_map.insert("a", 0);
    c_map.insert("b", 0);
    c_map.insert("c", 0);

    for d in V_DATA.split(",") {
        c_map.insert(d, c_map[d] + 1);
    }

    for k in c_map {
        println!("{}:{:>2}", k.0, k.1);
    }
}
