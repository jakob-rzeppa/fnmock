#[fnmock::fakeable]
fn named_lifetime<'a, 'b>(a: &'a str, b: &'b str) -> String {
    format!("{} {}", a, b)
}

#[test]
fn test_named_lifetime() {
    let value = "Test".to_string();
    let another = "Another".to_string();
    let result = named_lifetime(&value, &another);
    assert_eq!(result, "Test Another");
}

#[test]
fn test_named_lifetime_fake() {
    let value = "Test".to_string();
    let another = "Another".to_string();
    named_lifetime_fake().setup(|a, b| format!("{} {} fake modified", a, b));
    let result = named_lifetime(&value, &another);
    assert_eq!(result, "Test Another fake modified");
}
