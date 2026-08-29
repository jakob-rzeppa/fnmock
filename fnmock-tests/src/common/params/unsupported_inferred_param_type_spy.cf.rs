#[fnmock::spyable]
fn inferred_param(value: _) -> i32 {
    value
}

fn main() {}
