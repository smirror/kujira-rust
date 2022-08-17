fn main() {
    for i in 1..10 {
        let s = (1..10).map(|x| format!("{:3}", x * i)).collect::<Vec<String>>().join(",");
        println!("{}", s);
    }
}
