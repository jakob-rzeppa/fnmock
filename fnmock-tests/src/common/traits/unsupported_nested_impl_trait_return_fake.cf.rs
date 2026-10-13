//! `impl Trait` nested inside the return type is rejected like a top-level one: the fake
//! closure trait bound would have to name `Option<impl Trait>`, and the anonymous type inside
//! cannot be written out.

#[fnmock::fakeable]
fn returns_nested_impl_trait(value: i32) -> Option<impl std::fmt::Display> {
    Some(value)
}

fn main() {}
