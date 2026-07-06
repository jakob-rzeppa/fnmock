#[fnmock::fakeable]
fn by_value(a: String) -> String {
    a
}

#[test]
fn test_by_value() {
    let res = by_value("Test".to_string());
    assert_eq!(res, "Test");
}

#[test]
fn test_by_value_fake() {
    by_value_fake().setup(|a| format!("Fake {}", a));
    let res = by_value("Test".to_string());
    assert_eq!(res, "Fake Test");
}
