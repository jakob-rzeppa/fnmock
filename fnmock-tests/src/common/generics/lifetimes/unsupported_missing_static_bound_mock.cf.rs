//! A type parameter with no lifetime bound at all is rejected for the same reason as one with a
//! non-`'static` bound: both the fake and the spy store are keyed by `TypeId`, which requires `'static`.

#[fnmock::mockable]
fn missing_static_bound<T: std::fmt::Display>(value: T) -> String {
    format!("{value}")
}

fn main() {}
