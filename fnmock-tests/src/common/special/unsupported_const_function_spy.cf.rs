#[fnmock::spyable]
const fn const_function(value: i32) -> i32 {
    value + 1
}

fn main() {}
