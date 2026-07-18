//! Impl blocks on non-path types (tuples, references, ...) are not supported;
//! only simple paths (plus generics) are.

#[fnmock::fakeable]
impl (i32, i32) {
    fn sum(&self) -> i32 {
        self.0 + self.1
    }
}

fn main() {}
