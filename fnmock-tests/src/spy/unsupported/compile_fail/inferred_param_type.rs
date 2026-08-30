//! `_` as a parameter type cannot be named by the matcher either.

#[fnmock::spyable]
fn inferred_param_type(value: _) {
    let _ = value;
}

fn main() {}
