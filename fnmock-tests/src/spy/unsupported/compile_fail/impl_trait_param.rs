//! `impl Trait` denotes an anonymous type, so the matcher cannot name it in
//! `Predicate<..>` or in its `Fn(..)` bound.
//!
//! `#[fakeable]` already rejects this; the spy path did not check parameter
//! types at all and failed inside the generated module instead.

#[fnmock::spyable]
fn impl_trait_param(value: impl std::fmt::Display) {
    let _ = value;
}

fn main() {}
