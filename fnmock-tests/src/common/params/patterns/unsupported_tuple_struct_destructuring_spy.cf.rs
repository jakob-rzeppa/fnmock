//! Tuple-struct destructuring patterns are not supported: the call value
//! needs an owned binding, and a destructured tuple struct cannot be rebuilt
//! in the general case. This is a shared restriction, reported the same way
//! for both `#[fnmock::fakeable]` and `#[fnmock::spyable]`; see
//! unsupported/fake/tuple_struct_destructuring.rs for the fake counterpart.

pub struct Wrapper(pub i32);

#[fnmock::spyable]
fn tuple_struct_destructuring(Wrapper(inner): Wrapper) {
    let _ = inner;
}

fn main() {}
