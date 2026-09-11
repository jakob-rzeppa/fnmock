//! `const fn` methods in impl blocks are not supported: the fake lookup fnmock
//! injects cannot run in a const context.

pub struct Calculator;

#[fnmock::fakeable]
impl Calculator {
    const fn add(value: i32) -> i32 {
        value + 1
    }
}

fn main() {}
