//! Slice patterns (and other exotic patterns) are not supported for fake call
//! values; only plain identifiers and (nested) tuples of identifiers are.

#[fnmock::fakeable]
fn slice_pattern([a, b]: [i32; 2]) -> i32 {
    a + b
}

fn main() {}
