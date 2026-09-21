//! `#[deprecated]` written before `#[fnmock::spyable]` must survive expansion.

#![deny(deprecated)]

#[deprecated]
#[fnmock::spyable]
fn old_function(a: i32) -> i32 {
    a + 1
}

fn main() {
    old_function(1);
}
