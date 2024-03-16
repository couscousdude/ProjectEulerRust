//! [Problem 2](https://projecteuler.net/problem=2) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

const PHI: f64 = 1.618033988749;
const SQRT_5: f64 = 2.23606797749979;

fn fib_closed(n: u32) -> u32 {
    ((PHI.powf(n as f64) - (1.0_f64 - PHI).powf(n as f64)) / SQRT_5).round() as u32
}

fn compute(bound: u32) -> u32 {
    let mut count = 1;
    let mut sum = 0;

    let mut fib = 0;

    while fib < bound {
        fib = fib_closed(count);
        if fib % 2 == 0 {
            sum += fib;
        }
        count += 1;
    }

    sum
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
