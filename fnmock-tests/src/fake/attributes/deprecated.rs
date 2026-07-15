#[deprecated]
#[fnmock::fakeable]
fn deprecated(a: String) -> String {
    a
}

// Deprecated does not interfere with our fake implementation.

#[test]
fn test_deprecated() {
    #[allow(deprecated)]
    let res = deprecated("Test".to_string());
    assert_eq!(res, "Test");
}

#[test]
fn test_deprecated_fake() {
    deprecated_fake().setup(|a| format!("Fake {}", a));
    #[allow(deprecated)]
    let res = deprecated("Test".to_string());
    assert_eq!(res, "Fake Test");
}
