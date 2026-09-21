//! A `self` receiver on a free function is rejected; `self` receivers are only
//! supported on methods inside an inherent impl block.

#[fnmock::fakeable]
fn free_function(self) -> i32 {
    1
}

fn main() {}
