#[fnmock::fakeable]
fn static_lifetime<T: 'static>(a: T) -> T {
    a
}

#[test]
fn test_static_lifetime() {
    let res = static_lifetime("Test");
    assert_eq!(res, "Test");
}

#[test]
fn test_static_lifetime_fake() {
    static_lifetime_fake::<&'static str>().setup(|_| "Fake Test");
    let res = static_lifetime("Test");
    assert_eq!(res, "Fake Test");
}
