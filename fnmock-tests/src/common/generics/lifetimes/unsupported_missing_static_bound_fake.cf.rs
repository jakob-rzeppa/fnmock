//! A type parameter with no lifetime bound at all is rejected for the same reason as one with a
//! non-`'static` bound: the fake store is keyed by `TypeId`, which requires `'static`.

#[fnmock::fakeable]
fn missing_static_bound<T: std::fmt::Display>(value: T) -> String {
    format!("{value}")
}

fn main() {}
