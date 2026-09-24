//! The inferred type `_` is not supported as a parameter type: the fake
//! closure trait bound and the call record both need a concrete type.

#[fnmock::mockable]
fn inferred_param(value: _) -> i32 {
    value
}

fn main() {}
