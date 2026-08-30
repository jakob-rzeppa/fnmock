//! `#[must_use]` written after `#[fnmock::fakeable]` must survive expansion.

#![deny(unused_must_use)]

#[fnmock::fakeable]
#[must_use]
fn compute(a: i32) -> i32 {
    a + 1
}

fn main() {
    // We do not use the return value of `compute`, so this doesn't compile because of the `#![deny(unused_must_use)]`.
    compute(1);
}
