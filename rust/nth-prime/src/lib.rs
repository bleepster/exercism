fn is_prime(n: u32) -> bool {
    for i in 2..n {
        if n % i == 0 && i != n {
            return false;
        }
    }
    true
}

pub fn nth(n: u32) -> u32 {
    let mut found_primes = 0;
    let mut current_guess = 2;
    let mut last_found = current_guess;
    loop {
        if is_prime(current_guess) {
            found_primes += 1;
            last_found = current_guess;
        }
        if found_primes == (n + 1) {
            break;
        }
        current_guess += 1;
    }
    return current_guess;
}
