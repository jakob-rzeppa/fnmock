//! `#[must_use]` written before `#[fnmock::spyable]` must survive expansion.

#![deny(unused_must_use)]

#[must_use]
#[fnmock::fakeable]
fn compute(a: i32) -> i32 {
    a + 1
}

fn main() {
    // We do not use the return value of `compute`, so this doesn't compile because of the `#![deny(unused_must_use)]`.
    compute(1);
}
