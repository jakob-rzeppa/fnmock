//! An impl block declared inside a function's body is not supported. The
//! generated accessors would only be visible inside that body, so no test could
//! reach them; declare the impl block at module level instead.

pub fn outer() -> i32 {
    struct Counter;

    #[fnmock::fakeable]
    impl Counter {
        fn get(&self) -> i32 {
            1
        }
    }

    Counter.get()
}

fn main() {}
