//! `#[deprecated]` written before `#[fnmock::fakeable]` must survive expansion.

#![deny(deprecated)]

#[deprecated]
#[fnmock::fakeable]
fn old_function(a: i32) -> i32 {
    a + 1
}

fn main() {
    old_function(1);
}
