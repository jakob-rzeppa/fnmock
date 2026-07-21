//! `#[must_use]` above `#[fnmock::fakeable]` must survive expansion, exercising the same
//! attribute-preservation path as `deprecated_before_fakeable.rs` for a different attribute.

#![deny(unused_must_use)]

#[must_use]
#[fnmock::fakeable]
fn compute(a: i32) -> i32 {
    a + 1
}

fn main() {
    // We do not use the return value of `compute`, so this doesnt compile because of the `#![deny(unused_must_use)]`.
    compute(1);
}
