#[fnmock::fakeable]
fn tuple_destructuring((left, right): (String, String)) -> String {
    format!("{}{}", left, right)
}

#[test]
fn test_tuple_destructuring() {
    let result = tuple_destructuring(("Test".to_string(), " Value".to_string()));
    assert_eq!(result, "Test Value");
}

#[test]
fn test_tuple_destructuring_fake() {
    tuple_destructuring_fake().setup(|(left, right)| format!("Fake {}{}", left, right));

    let result = tuple_destructuring(("Test".to_string(), " Value".to_string()));
    assert_eq!(result, "Fake Test Value");
}
