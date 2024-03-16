//! [Problem 5](https://projecteuler.net/problem=5) solver.

#![warn(
    bad_style,
    unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::collections::HashSet;

const CRITERIA: [u8; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

fn factorize(num: u32, set: &mut HashSet<u8>) {
    match num {
        2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 => {
            let _ = set.insert(num as u8);
        }
        _ => {
            for i in 2..9 {
                if num % i == 0 {
                    factorize(i, set);
                    factorize(num / i, set);
                }
            }
        }
    }
}

fn check_num(n: u32) -> bool {
    let mut set: HashSet<u8> = HashSet::with_capacity(8);
    factorize(n, &mut set);

    CRITERIA.len() == set.len() && CRITERIA.iter().all(|&item| set.contains(&item))
}

fn compute(n: u32) -> u32 {
    let mut num = 20;

    loop {
        if check_num(num) {
            break num;
        }
        num += 20;
    }
}

fn solve() -> String {
    compute(20).to_string()
}

common::problem!("232792560", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn evenly_dividable_below_10() {
        assert_eq!(2520, super::compute(10));
    }
}
