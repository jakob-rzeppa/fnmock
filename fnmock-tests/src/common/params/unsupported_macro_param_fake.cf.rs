//! A macro invocation in parameter-pattern position (`m!()`) is rejected: the generated
//! fake forwards each parameter by name, and a macro pattern has no identifier to forward.

#[fnmock::fakeable]
fn macro_pattern(m!(): i32) -> i32 {
    0
}

fn main() {}
