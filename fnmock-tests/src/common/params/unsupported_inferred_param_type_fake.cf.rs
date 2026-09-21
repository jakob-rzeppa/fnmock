//! The inferred type `_` is not supported as a parameter type: the fake
//! closure trait bound needs a concrete type to build the `Fn` bound from.

#[fnmock::fakeable]
fn inferred_param(value: _) -> i32 {
    value
}

fn main() {}
