//! `impl Trait` is not supported in a fakeable function signature: the fake
//! closure trait bound needs to name the parameter type explicitly, and
//! `impl Trait` denotes an anonymous type that cannot be written out.

#[fnmock::fakeable]
fn takes_impl_trait(value: impl std::fmt::Display) -> String {
    value.to_string()
}

fn main() {}
