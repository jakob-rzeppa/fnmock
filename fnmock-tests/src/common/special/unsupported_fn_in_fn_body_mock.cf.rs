//! A function declared inside another function's body is not supported. The
//! generated accessor would only be visible inside that body, so no test could
//! reach it; declare the function at module level instead.

pub fn outer() -> i32 {
    #[fnmock::mockable]
    fn inner(a: i32) -> i32 {
        a + 1
    }

    inner(1)
}

fn main() {}
