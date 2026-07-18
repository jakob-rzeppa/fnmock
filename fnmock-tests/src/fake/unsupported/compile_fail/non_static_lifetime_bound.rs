//! Non-'static lifetime bounds on generic parameters are not supported: the
//! fake store keys fakes by `TypeId`, which requires 'static types.

#[fnmock::fakeable]
fn bounded<'a, T: 'a + std::fmt::Display>(value: &'a T) -> String {
    format!("{value}")
}

fn main() {}
