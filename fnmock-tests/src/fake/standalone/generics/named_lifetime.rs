#[fnmock::fakeable]
fn named_lifetime<'a>(a: &'a str) -> String {
    a.to_string()
}

#[test]
fn test_named_lifetime() {
    let value = "Test".to_string();
    let result = named_lifetime(&value);
    assert_eq!(result, "Test");
}

#[test]
fn test_named_lifetime_fake() {
    let value = "Test".to_string();
    named_lifetime_fake().setup(|a| format!("{} fake modified", a));
    let result = named_lifetime(&value);
    assert_eq!(result, "Test fake modified");
}
