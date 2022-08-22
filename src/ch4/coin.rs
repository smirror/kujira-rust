use crate::Coin::*;

enum Coin {
    Coin1(isize),
    Coin5(isize),
    Coin10(isize),
    Coin50(isize),
    Coin100(isize),
    Coin500(isize),
}

impl Coin {
    fn calc_price(&self) -> isize {
        match *self {
            Coin1(v) => v,
            Coin5(v) => 5 * v,
            Coin10(v) => 10 * v,
            Coin50(v) => 50 * v,
            Coin100(v) => 100 * v,
            Coin500(v) => 500 * v,
        }
    }
}

fn main() {
    let wallet: Vec<Coin> = vec![
        Coin1(3),
        Coin5(10),
        Coin10(5),
        Coin50(1),
        Coin100(1),
        Coin500(5),
    ];
    let total = wallet.iter().fold(0, |sum, v| sum + v.calc_price());
    println!("財布の合計は{}円です", total);
}
