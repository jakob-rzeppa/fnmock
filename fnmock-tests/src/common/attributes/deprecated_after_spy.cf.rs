//! `#[deprecated]` written below `#[fnmock::spyable]` must survive expansion.

#![deny(deprecated)]

#[fnmock::spyable]
#[deprecated]
fn old_function(a: i32) -> i32 {
    a + 1
}

fn main() {
    old_function(1);
}
