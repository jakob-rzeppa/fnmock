#[inline]
#[fnmock::fakeable]
fn inline(a: String) -> String {
    a
}

// Inline does not interfere with our fake implementation.

#[test]
fn test_inline() {
    let res = inline("Test".to_string());
    assert_eq!(res, "Test");
}

#[test]
fn test_inline_fake() {
    inline_fake().setup(|a| format!("Fake {}", a));
    let res = inline("Test".to_string());
    assert_eq!(res, "Fake Test");
}
