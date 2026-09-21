//! Tuple-struct destructuring patterns are not supported: the fake closure
//! receives whole values, and a destructured tuple struct cannot be rebuilt in
//! the general case.

pub struct Wrapper(pub i32);

#[fnmock::fakeable]
fn tuple_struct_destructuring(Wrapper(inner): Wrapper) -> i32 {
    inner
}

fn main() {}
