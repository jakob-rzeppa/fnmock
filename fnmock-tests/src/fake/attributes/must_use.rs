#[must_use]
#[fnmock::fakeable]
fn must_use(a: String) -> String {
    a
}

// must_use does not interfere with our fake implementation.

#[test]
fn test_must_use() {
    let res = must_use("Test".to_string());
    assert_eq!(res, "Test");
}

#[test]
fn test_must_use_fake() {
    must_use_fake().setup(|a| format!("Fake {}", a));
    let res = must_use("Test".to_string());
    assert_eq!(res, "Fake Test");
}
