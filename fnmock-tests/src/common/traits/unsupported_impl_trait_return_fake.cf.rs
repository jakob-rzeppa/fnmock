//! `impl Trait` is not supported in a fakeable function signature: the fake
//! closure trait bound needs to name the return type explicitly, and
//! `impl Trait` denotes an anonymous type that cannot be written out.

#[fnmock::fakeable]
fn returns_impl_trait(value: i32) -> impl std::fmt::Display {
    value
}

fn main() {}
