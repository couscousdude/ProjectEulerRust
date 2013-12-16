#[crate_type = "rlib"];

extern mod math;

use math::prime;

pub static EXPECTED_ANSWER: &'static str = "104743";

pub fn solve() -> ~str {
    return prime::nth(10000).to_str();
}
