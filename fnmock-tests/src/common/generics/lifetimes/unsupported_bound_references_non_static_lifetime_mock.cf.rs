#[fnmock::mockable]
fn bound_references_non_static_lifetime<'a, T: Into<&'a str> + 'static>(value: T) -> usize {
    let s: &'a str = value.into();
    s.len()
}

fn main() {}
