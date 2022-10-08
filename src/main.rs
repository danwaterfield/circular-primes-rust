
fn main() {
println!("there are {} circular primes below 1 million", count_circular_primes(1_000_000));
}


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
// The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and 719, are themselves prime.
// There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.

// How many circular primes are there below one million?
// this'll be the test case
// we want the user to be able to input a number and the program to return the number of circular primes below that number

// we also want to cache the results of the prime function, so a request coming in will 
// check the cache first, and if it's not there, it'll calculate it and add it to the cache
// we'll use a hashmap to do this

