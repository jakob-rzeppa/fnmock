//! A spy records calls into a `thread_local!` store, which cannot run in a
//! const context.

#[fnmock::spyable]
const fn const_function(a: i32) -> i32 {
    a
}

fn main() {}
