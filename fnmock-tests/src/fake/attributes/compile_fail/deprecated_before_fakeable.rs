//! `#[deprecated]` written above `#[fnmock::fakeable]` must survive expansion.
//! `#![deny(deprecated)]` turns the (normally warn-only) deprecation lint into a hard error, so if
//! fnmock ever dropped the attribute while re-emitting the function this file would compile
//! instead of failing.

#![deny(deprecated)]

#[deprecated]
#[fnmock::fakeable]
fn old_function(a: i32) -> i32 {
    a + 1
}

fn main() {
    old_function(1);
}
