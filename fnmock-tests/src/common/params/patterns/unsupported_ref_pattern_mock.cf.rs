//! `ref` patterns are not supported: both the fake closure and the call record
//! need owned values, and a value cannot be recovered from a reference in the
//! general case.

#[fnmock::mockable]
fn reference_pattern((ref left, right): (i32, i32)) -> i32 {
    left + right
}

fn main() {}
