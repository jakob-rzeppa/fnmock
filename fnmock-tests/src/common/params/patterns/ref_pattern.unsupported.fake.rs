//! `ref` patterns are not supported: the fake closure needs owned values, and
//! a value cannot be recovered from a reference in the general case.

#[fnmock::fakeable]
fn reference_pattern((ref left, right): (i32, i32)) -> i32 {
    left + right
}

fn main() {}
