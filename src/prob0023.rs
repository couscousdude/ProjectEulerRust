#[crate_type = "rlib"];

extern mod math;

use std::vec;
use math::prime;

pub static EXPECTED_ANSWER: &'static str = "4179871";

#[inline(always)]
fn is_abundant(n: uint) -> bool {
    prime::sum_of_proper_divisors(n) > n
}

pub fn solve() -> ~str {
    let max_num = 28123;

    let abundant = vec::build(Some(max_num + 1), |push| {
        for n in range(2, max_num + 1) {
            if is_abundant(n) { push(n); }
        }
    });

    let mut sum_of_sum_abundant = 0;
    let mut is_sum_abundant = vec::from_elem(max_num + 1, false);
    for (i, &a) in abundant.iter().enumerate() {
        for &b in abundant.tailn(i).iter() {
            let s = a + b;
            if s > max_num { break; }
            if !is_sum_abundant[s] { sum_of_sum_abundant += s; }
            is_sum_abundant[s] = true;
        }
    }

    let sum_of_all_int = (1 + max_num) * max_num / 2;

    return (sum_of_all_int - sum_of_sum_abundant).to_str();
}
