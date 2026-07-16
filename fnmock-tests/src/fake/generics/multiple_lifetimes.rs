#[fnmock::fakeable]
fn multiple_lifetimes<'a, 'b>(a: &'a str, b: &'b str) -> String {
    format!("{} {}", a, b)
}

#[test]
fn test_multiple_lifetimes() {
    let value = "Test".to_string();
    let another = "Another".to_string();
    let result = multiple_lifetimes(&value, &another);
    assert_eq!(result, "Test Another");
}

#[test]
fn test_multiple_lifetimes_fake() {
    let value = "Test".to_string();
    let another = "Another".to_string();
    multiple_lifetimes_fake().setup(|a, b| format!("{} {} fake modified", a, b));
    let result = multiple_lifetimes(&value, &another);
    assert_eq!(result, "Test Another fake modified");
}
