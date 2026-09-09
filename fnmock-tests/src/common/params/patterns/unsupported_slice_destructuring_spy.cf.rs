//! Slice destructuring in parameter position is rejected for the same reason as tuple
//! destructuring: `#[fnmock::spyable]` needs a plain identifier per parameter to record and
//! match it under. Take the array by a plain binding (`values: [i32; 2]`) instead.

#[fnmock::spyable]
fn slice_destructuring([first, second]: [i32; 2]) -> i32 {
    first + second
}

fn main() {}
