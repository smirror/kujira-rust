fn encrpt(text: &str, shift: i16) -> String {
    let a = 'a' as i16;
    let is_az = |c| 'a' <= c && c <= 'z';
    let conv = |c| (((c - a + shift) % 26 + a) as u8) as char;
    let enc1 = |c| if is_az(c) { conv(c as i16) } else { c };
    return text.chars().map(|c| enc1(c)).collect();
}

fn main() {
    let text = "hello, world!";
    let shift = 3;
    let enc = encrpt(text, shift);
    let dec = encrpt(&enc, -shift);
    println!("{} => {}", enc, dec);
}
