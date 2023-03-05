fn is_prime(x: u64) -> bool {
    for i in 2..x {
        if x % i == 0 {
            return false;
        }
    }

    true
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut x = n;
    let mut prime_factors: Vec<u64> = vec![];

    while x > 1 {
        for i in 2..=x {
            if x % i == 0 && is_prime(i) {
                prime_factors.push(i);
                x = x / i;
                break;
            }
        }
    }

    prime_factors
}
