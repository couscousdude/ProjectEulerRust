//! [Problem 3](https://projecteuler.net/problem=3) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use num_integer::sqrt;

fn largest_prime_factor(num: u64, largest: &mut u64) {
    let mut is_prime = true;
    for i in 2..sqrt(num) + 1 {
        if num % i == 0 {
            largest_prime_factor(i, largest);
            largest_prime_factor(num / i, largest);
            is_prime = false;
        }
    }

    if is_prime && num > *largest {
        *largest = num;
    }
}

fn compute(n: u64) -> u64 {
    let mut largest = 0;
    largest_prime_factor(n, &mut largest);
    largest
}

fn solve() -> String {
    compute(600851475143).to_string()
}
common::problem!("6857", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn factorize_13195() {
        assert_eq!(29, super::compute(13195));
    }
}
