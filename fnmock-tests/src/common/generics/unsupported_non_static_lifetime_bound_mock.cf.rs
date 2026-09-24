//! Non-'static lifetime bounds on generic parameters are not supported: both the
//! fake and the spy store are keyed by `TypeId`, which requires 'static types.

#[fnmock::mockable]
fn bounded<'a, T: 'a + std::fmt::Display>(value: &'a T) -> String {
    format!("{value}")
}

fn main() {}
