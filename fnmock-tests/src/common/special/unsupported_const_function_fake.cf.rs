//! `const fn` is not supported: the fake lookup fnmock injects cannot run in a
//! const context.

#[fnmock::fakeable]
const fn const_function(value: i32) -> i32 {
    value + 1
}

fn main() {}
