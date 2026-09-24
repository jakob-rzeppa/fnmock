//! A mock is the intersection of what fake and spy support. Spy has no return
//! type to name, but fake's closure bound has to name the return type
//! explicitly, and `impl Trait` denotes an anonymous type that cannot be
//! written out. `#[fnmock::mockable]` rejects it with fake's message.

#[fnmock::mockable]
fn returns_impl_trait(value: i32) -> impl std::fmt::Display {
    value
}

fn main() {}
