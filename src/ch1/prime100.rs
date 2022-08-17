fn is_prime(n: u128) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return  false;
        }
    }
    return  true;
}

fn get_primes(n: u128) -> Vec<u128> {
    let mut primes = Vec::new();
    let mut i = 2;
    let mut count = 0;
    while count < n {
        if is_prime(i) {
            primes.push(i);
            count += 1;
        }
        i += 1;
    }
    primes
}

fn main() {
    println!("{:?}", get_primes(100));
}
