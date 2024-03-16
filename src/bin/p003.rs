//! [Problem 3](https://projecteuler.net/problem=3) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::collections::HashSet;

use num_integer::sqrt;

fn prime_factorize(num: u64, collection: &mut HashSet<u64>) {
    let mut is_prime = true;
    for i in 2..sqrt(num) + 1 {
        if num % i == 0 {
            prime_factorize(i, collection);
            prime_factorize(num / i, collection);
            is_prime = false;
        }
    }

    if is_prime {
        let _ = collection.insert(num);
    }
}

fn compute(n: u64) -> u64 {
    let mut factors: HashSet<u64> = HashSet::new();
    prime_factorize(n, &mut factors);

    let mut largest = 0;
    for item in factors {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn solve() -> String {
    compute(600851475143).to_string()

    // compute(789).to_string()
}
common::problem!("6857", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn factorize_13195() {
        assert_eq!(29, super::compute(13195));
    }
}
