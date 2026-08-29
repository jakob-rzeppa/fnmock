//! Struct destructuring patterns are not supported: the fake closure receives
//! whole values, and a destructured struct cannot be rebuilt in the general
//! case (e.g. private fields).

pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[fnmock::fakeable]
fn struct_destructuring(Point { x, y }: Point) -> i32 {
    x + y
}

fn main() {}
