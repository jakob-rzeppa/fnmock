//! Unlike a fake, whose closure reproduces the parameter pattern, a spy needs
//! exactly one plain identifier per parameter.

struct Point {
    x: i32,
    y: i32,
}

#[fnmock::spyable]
fn struct_destructuring_param(Point { x, y }: Point) {
    let _ = (x, y);
}

fn main() {}
