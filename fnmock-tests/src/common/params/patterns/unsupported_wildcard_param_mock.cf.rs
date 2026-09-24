//! Ignored (`_`) parameters are not supported: the generated mock forwards every
//! parameter to both the fake closure and the call record, and `_` has no name to
//! forward. Use `_name` instead.

#[fnmock::mockable]
fn ignored(_: String, value: String) -> String {
    value
}

fn main() {}
