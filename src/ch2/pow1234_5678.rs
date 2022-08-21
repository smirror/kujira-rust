// not work!
// thread 'main' panicked at 'attempt to multiply with overflow'
fn main() {
    let v:u128 = 1234;
    print!("{}", v.pow(5678));
}
