//! [Problem 4](https://projecteuler.net/problem=4) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn check_palindrome(num: u32) -> bool {
    let mut n = num;
    let mut rev = 0;
    while n != 0 {
        rev = rev * 10 + n % 10;
        n /= 10;
    }
    rev == num
}

fn compute(min: u32, max: u32) -> u32 {
    let mut largest = 0;
    for i in min..(max + 1) {
        for k in min..(max + 1) {
            if check_palindrome(i * k) && i * k > largest {
                largest = i * k;
            }
        }
    }
    largest
}

fn solve() -> String {
    compute(100, 999).to_string()
}

common::problem!("906609", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn two_digits_palindromic() {
        assert_eq!(9009, super::compute(10, 99));
    }
}
