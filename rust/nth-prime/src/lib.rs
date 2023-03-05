pub fn is_prime(x: u32) -> bool {
    for i in 2..x - 1 {
        if x % i == 0 {
            return false;
        }
    }

    true
}

pub fn nth(n: u32) -> u32 {
    let mut num_primes = 0;
    let mut current_prime = 2;
    let mut current_num = 2;

    while num_primes < n {
        current_num += 1;

        if is_prime(current_num) {
            current_prime = current_num;
            num_primes += 1;
        }
    }

    current_prime
}
