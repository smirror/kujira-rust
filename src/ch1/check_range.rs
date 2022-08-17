fn main() {
    println!("符号あり整数");
    println!("{}~{}", i8::MIN, i8::MAX);
    println!("{}~{}", i16::MIN, i16::MAX);
    println!("{}~{}", i32::MIN, i32::MAX);
    println!("{}~{}", i64::MIN, i64::MAX);
    println!("{}~{}", i128::MIN, i128::MAX);

    println!("符号なし整数");
    println!("{}~{}", u8::MIN, u8::MAX);
    println!("{}~{}", u16::MIN, u16::MAX);
    println!("{}~{}", u32::MIN, u32::MAX);
    println!("{}~{}", u64::MIN, u64::MAX);
    println!("{}~{}", u128::MIN, u128::MAX);

    println!("符号あり浮動小数点");
    println!("{}~{}", f32::MIN, f32::MAX);
    println!("{}~{}", f64::MIN, f64::MAX);


    println!("実行環境で変わる整数");
    println!("{}~{}", isize::MIN, isize::MAX);
    println!("{}~{}", usize::MIN, usize::MAX);
}
