//! A mock is the intersection of what fake and spy support. Fake reproduces a
//! slice-destructuring pattern in its closure just fine, but spy has no name to
//! match the parameter under, so `#[fnmock::mockable]` rejects it with spy's
//! message. Take the array by a plain binding (`values: [i32; 2]`) instead.

#[fnmock::mockable]
fn slice_destructuring([first, second]: [i32; 2]) -> i32 {
    first + second
}

fn main() {}
