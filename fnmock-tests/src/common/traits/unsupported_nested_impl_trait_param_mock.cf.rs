//! `impl Trait` nested inside a parameter type is rejected like a top-level one: both the fake
//! closure trait bound and the spy's `Predicate<..>` bound would have to name
//! `Vec<impl Trait>`, and the anonymous type inside cannot be written out.

#[fnmock::mockable]
fn takes_nested_impl_trait(values: Vec<impl std::fmt::Display>) -> usize {
    values.len()
}

fn main() {}
