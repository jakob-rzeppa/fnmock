//! `#[deprecated]` written below `#[fnmock::fakeable]` must survive expansion.

#![deny(deprecated)]

#[fnmock::fakeable]
#[deprecated]
fn old_function(a: i32) -> i32 {
    a + 1
}

fn main() {
    old_function(1);
}
