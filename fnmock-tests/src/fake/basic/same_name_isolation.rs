//! Two fakeable free functions with the same name, defined in different modules, must have
//! independent fakes and real bodies — the free-function analog of
//! `impl_block/same_method_name_isolation.rs`. Isolation here falls out of Rust's own module
//! system (the generated fake module nests inside the function's own module), but nothing pinned
//! that until now.

mod first {
    #[fnmock::fakeable]
    pub fn fetch(a: i32) -> i32 {
        a + 1
    }
}

mod second {
    #[fnmock::fakeable]
    pub fn fetch(a: i32) -> i32 {
        a + 100
    }
}

#[test]
fn test_real_bodies_are_independent() {
    assert_eq!(first::fetch(1), 2);
    assert_eq!(second::fetch(1), 101);
}

#[test]
fn test_fakes_are_independent() {
    first::fetch_fake().setup(|a| a + 1000);
    assert_eq!(first::fetch(1), 1001);
    assert_eq!(second::fetch(1), 101);

    second::fetch_fake().setup(|a| a + 2000);
    assert_eq!(first::fetch(1), 1001);
    assert_eq!(second::fetch(1), 2001);
}
