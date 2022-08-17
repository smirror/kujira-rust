fn encrpt(text: &str, shift: i16) -> String {
    let code_a = 'a' as i16;
    let code_z = 'z' as i16;
    let mut result = String::new();
    for c in text.chars() {
        let mut code = c as i16;
        if code_a <= code && code <= code_z {
            code = (code - code_a + shift) % 26 + code_a;
        }
        result.push(code as u8 as char);
    }
    result
}

fn main() {
    let text = "hello, world!";
    let shift = 3;
    let enc = encrpt(text, shift);
    let dec = encrpt(&enc, -shift);
    println!("{} => {}", enc, dec);
}
