//! `#[fakeable]` can only be applied to functions and impl blocks, not to
//! other items like structs.

#[fnmock::fakeable]
pub struct NotAFunction {
    pub value: i32,
}

fn main() {}
