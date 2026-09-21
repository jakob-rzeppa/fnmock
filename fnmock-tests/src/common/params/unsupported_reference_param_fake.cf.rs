//! A reference pattern (`&x`) in parameter position is not one of the explicitly-handled
//! pattern kinds, so it falls through to the catch-all "Unsupported pattern type" arm: the
//! generated fake forwards each parameter by name, and a reference pattern has no plain
//! identifier to forward. Use a plain binding (e.g. `x: &i32`) instead.

#[fnmock::fakeable]
fn reference_pattern(&x: &i32) -> i32 {
    x
}

fn main() {}
