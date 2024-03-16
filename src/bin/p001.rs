//! [Problem 1](https://projecteuler.net/problem=1) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn compute(bound: u32) -> u32 {
    let mut result = 0;

    for n in 1..bound {
        if n % 3 == 0 && n % 5 != 0 {
            result += n;
        } else if n % 5 == 0 {
            result += n;
        }
    }
    result
}

fn solve() -> String {
    compute(1000).to_string()
}

common::problem!("233168", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn sum_below_ten() {
        assert_eq!(23, super::compute(10));
    }
}
