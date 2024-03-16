//! [Problem 2](https://projecteuler.net/problem=2) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use num_integer::Integer;

fn fib(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        let ans = fib(n - 1) + fib(n - 2);
        ans
    }
}

fn compute(bound: u32) -> u32 {
    let mut count = 1;
    let mut sum = 0;
    loop {
        let num = fib(count);
        if num > bound {
            break sum;
        }
        if num % 2 == 0 {
            sum += num;
        }
        count += 1;
    }
}

fn solve() -> String {
    compute(4000000).to_string()
}
common::problem!("4613732", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn sum_below_90() {
        assert_eq!(44, super::compute(90));
    }
}
