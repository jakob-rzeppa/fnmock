//! Struct destructuring patterns are not supported: both the fake closure and
//! the call record receive whole values, and a destructured struct cannot be
//! rebuilt in the general case (e.g. private fields).

pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[fnmock::mockable]
fn struct_destructuring(Point { x, y }: Point) -> i32 {
    x + y
}

fn main() {}
