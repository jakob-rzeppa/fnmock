//! `#[deprecated]` written before `#[fnmock::mockable]` must survive expansion.

#![deny(deprecated)]

#[deprecated]
#[fnmock::mockable]
fn old_function(a: i32) -> i32 {
    a + 1
}

fn main() {
    old_function(1);
}
