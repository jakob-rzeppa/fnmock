//! Two `#[fnmock::fakeable]` impl blocks for same-named structs from different modules
//! (`a::Config` vs `b::Config`), both written at the same enclosing scope,
//! must have independent fakes and real bodies.

mod a {
    pub struct Config;
}

mod b {
    pub struct Config;
}

#[fnmock::fakeable]
impl a::Config {
    fn basic(&self) -> i32 {
        1
    }
}

#[fnmock::fakeable]
impl b::Config {
    fn basic(&self) -> i32 {
        2
    }
}

#[test]
fn test_real_bodies_are_independent() {
    assert_eq!(a::Config.basic(), 1);
    assert_eq!(b::Config.basic(), 2);
}

#[test]
fn test_fakes_are_independent() {
    a::Config::basic_fake().setup(|_| 10);
    assert_eq!(a::Config.basic(), 10);
    assert_eq!(b::Config.basic(), 2);

    b::Config::basic_fake().setup(|_| 20);
    assert_eq!(a::Config.basic(), 10);
    assert_eq!(b::Config.basic(), 20);
}
