//! Ignored (`_`) parameters are not supported: the generated fake needs to
//! forward every parameter to the fake closure, and `_` has no name to forward.
//! Use `_name` instead.

#[fnmock::fakeable]
fn ignored(_: String, value: String) -> String {
    value
}

fn main() {}
