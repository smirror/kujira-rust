fn main() {
    let price: i64 = 3950;
    let count500: i64 = 10;
    let count100: i64 = 3;
    let count50: i64 = 10;
    for i500 in 0..(count500 + 1) {
        for i100 in 0..(count100 + 1) {
            for i50 in 0..(count50 + 1) {
                let total = 500 * i500 + 100 * i100 + 50 * i50;
                if price == total {
                    println!("500円玉{}枚, 100円玉{}枚, 50円玉{}枚", i500, i100, i50);
                }
            }
        }
    }
}
