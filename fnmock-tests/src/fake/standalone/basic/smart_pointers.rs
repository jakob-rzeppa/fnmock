#[fnmock::fakeable]
fn smart_pointers(a: Box<String>) -> String {
    a.to_string()
}

#[test]
fn test_smart_pointers() {
    let value = "Test".to_string();
    let result = smart_pointers(Box::new(value));
    assert_eq!(result, "Test");
}

#[test]
fn test_smart_pointers_fake() {
    let value = "Test".to_string();
    smart_pointers_fake().setup(|a| format!("{} fake modified", a));
    let result = smart_pointers(Box::new(value));
    assert_eq!(result, "Test fake modified");
}
