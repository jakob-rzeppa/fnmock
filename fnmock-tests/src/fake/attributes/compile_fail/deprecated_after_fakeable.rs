//! `#[deprecated]` written below `#[fnmock::fakeable]` (the reverse order of
//! `deprecated_before_fakeable.rs`) must also survive expansion.

#![deny(deprecated)]

#[fnmock::fakeable]
#[deprecated]
fn old_function(a: i32) -> i32 {
    a + 1
}

fn main() {
    old_function(1);
}
