//! Tuple-struct destructuring patterns are not supported: both the fake closure
//! and the call record receive whole values, and a destructured tuple struct
//! cannot be rebuilt in the general case.

pub struct Wrapper(pub i32);

#[fnmock::mockable]
fn tuple_struct_destructuring(Wrapper(inner): Wrapper) -> i32 {
    inner
}

fn main() {}
