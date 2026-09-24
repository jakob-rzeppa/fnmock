//! `#[fnmock::mockable]` can only be applied to functions and impl blocks, not to
//! other items like structs.

#[fnmock::mockable]
pub struct NotAFunction {
    pub value: i32,
}

fn main() {}
