fn is_prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn is_circular_prime(n: u64) -> bool {
    let mut n = n;
    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.reverse();
    let mut i = 0;
    while i < digits.len() {
        let mut j = 0;
        let mut num = 0;
        while j < digits.len() {
            num = num * 10 + digits[(i + j) % digits.len()];
            j += 1;
        }
        if !is_prime(num) {
            return false;
        }
        i += 1;
    }
    true
}

fn count_circular_primes(n: u64) -> u64 {
    let mut count = 0;
    let mut i = 2;
    while i < n {
        if is_circular_prime(i) {
            count += 1;
        }
        i += 1;
    }
    count
}