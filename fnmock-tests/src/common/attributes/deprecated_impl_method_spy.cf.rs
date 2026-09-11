#![deny(deprecated)]

struct Widget;

#[fnmock::spyable]
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
