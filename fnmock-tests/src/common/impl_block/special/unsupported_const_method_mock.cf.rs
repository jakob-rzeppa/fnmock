//! `const fn` methods in impl blocks are not supported: the code fnmock injects
//! for a mock (a call record and a fake lookup) cannot run in a const context.

pub struct Calculator;

#[fnmock::mockable]
impl Calculator {
    const fn add(value: i32) -> i32 {
        value + 1
    }
}

fn main() {}
