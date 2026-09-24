//! A reference pattern (`&x`) in parameter position is rejected: the generated mock
//! forwards each parameter by name, and a reference pattern has no plain identifier to
//! forward. Use a plain binding (e.g. `x: &i32`) instead.

#[fnmock::mockable]
fn reference_pattern(&x: &i32) -> i32 {
    x
}

fn main() {}
