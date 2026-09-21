#[fnmock::spyable]
fn takes_impl_trait(value: impl std::fmt::Display) -> String {
    value.to_string()
}

fn main() {}
