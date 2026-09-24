//! `const fn` is not supported: it fails in item_info before either the fake
//! or the spy half runs, since the code fnmock injects for a mock (a call
//! record and a fake lookup) cannot run in a const context.

#[fnmock::mockable]
const fn const_function(value: i32) -> i32 {
    value + 1
}

fn main() {}
