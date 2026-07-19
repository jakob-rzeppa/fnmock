//! `#[deprecated]` on a method inside a `#[fnmock::fakeable]` impl block must survive expansion
//! too; the impl-block expansion path clones and re-emits methods separately from the free
//! function path, so it needs its own regression coverage.

#![deny(deprecated)]

struct Widget;

#[fnmock::fakeable]
impl Widget {
    #[deprecated]
    fn old_method(&self, a: i32) -> i32 {
        a + 1
    }
}

fn main() {
    let widget = Widget;
    widget.old_method(1);
}
