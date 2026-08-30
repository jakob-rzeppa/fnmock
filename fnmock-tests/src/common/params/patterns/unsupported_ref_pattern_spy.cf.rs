//! `ref` patterns are not supported: the call value needs an owned binding,
//! and a value cannot be recovered from a reference in the general case. This
//! is a shared restriction, reported the same way for both `#[fnmock::fakeable]`
//! and `#[fnmock::spyable]`; see unsupported/fake/ref_pattern.rs for the fake
//! counterpart.

#[fnmock::spyable]
fn reference_pattern(ref value: i32) {
    let _ = value;
}

fn main() {}
