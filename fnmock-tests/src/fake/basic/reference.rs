#[fnmock::fakeable]
fn reference(a: &str) -> String {
    a.to_string()
}

#[test]
fn test_reference() {
    let value = "Test".to_string();
    let result = reference(&value);
    assert_eq!(result, "Test");
}

#[test]
fn test_reference_fake() {
    let value = "Test".to_string();
    reference_fake().setup(|a| format!("{} fake modified", a));
    let result = reference(&value);
    assert_eq!(result, "Test fake modified");
}
