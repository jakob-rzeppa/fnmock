#[fnmock::fakeable]
fn mutable_pattern(mut val: String) -> String {
    val.push_str(" prefix");
    val
}

#[test]
fn test_mutable_pattern() {
    let result = mutable_pattern("Test".to_string());
    assert_eq!(result, "Test prefix");
}

#[test]
fn test_mutable_pattern_fake() {
    mutable_pattern_fake().setup(|mut val| {
        val.push_str(" Fake prefix");
        val
    });

    let result = mutable_pattern("Test".to_string());
    assert_eq!(result, "Test Fake prefix");
}

#[fnmock::fakeable]
fn mutable_pattern_tuple((mut left, right): (String, String)) -> String {
    left.push_str(&right);
    left
}

#[test]
fn test_mutable_pattern_tuple() {
    let result = mutable_pattern_tuple(("Test".to_string(), " Value".to_string()));
    assert_eq!(result, "Test Value");
}

#[test]
fn test_mutable_pattern_tuple_fake() {
    mutable_pattern_tuple_fake().setup(|(mut left, right)| {
        left.push_str(" Fake");
        left.push_str(&right);
        left
    });

    let result = mutable_pattern_tuple(("Test".to_string(), " Value".to_string()));
    assert_eq!(result, "Test Fake Value");
}
