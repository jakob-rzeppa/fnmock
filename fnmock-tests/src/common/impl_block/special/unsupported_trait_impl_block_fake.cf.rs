//! Trait impl blocks (`impl Trait for Type`) are not supported; only inherent
//! impl blocks (`impl Type { ... }`) are.

pub trait Greet {
    fn greet(&self) -> String;
}

pub struct Greeter;

#[fnmock::fakeable]
impl Greet for Greeter {
    fn greet(&self) -> String {
        "hello".to_string()
    }
}

fn main() {}
