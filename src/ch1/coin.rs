fn main() {
    let price = 3950;
    for i500 in 0..11 {
        for i100 in 0..4 {
            for i50 in 0..11 {
                let total = 500 * i500 + 100 * i100 + 50 * i50;
                if price == total {
                    println!("500円玉{}枚, 100円玉{}枚, 50円玉{}枚", i500, i100, i50);
                }
            }
        }
    }
}
