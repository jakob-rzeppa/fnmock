use std::fmt::Display;

#[fnmock::fakeable]
fn const_generics_mixed<T: Display + 'static, const C: usize>(value: T) -> String {
    format!("{} {}", value, C)
}

#[test]
fn test_const_generics_mixed() {
    let res = const_generics_mixed::<String, 5>("Test".to_string());
    assert_eq!(res, "Test 5");
}

#[test]
fn test_const_generics_mixed_fake() {
    const_generics_mixed_fake::<String, 5>().setup(|value| format!("Fake {} 5", value));

    let res = const_generics_mixed::<String, 5>("Test".to_string());
    assert_eq!(res, "Fake Test 5");
}

#[test]
fn test_const_generics_mixed_value_isolation() {
    const_generics_mixed_fake::<String, 5>().setup(|value| format!("Fake {}", value));

    // A different const value falls back to the real implementation.
    let res = const_generics_mixed::<String, 7>("Test".to_string());
    assert_eq!(res, "Test 7");

    // A different type parameter also falls back to the real implementation, even though
    // the const value matches.
    let res = const_generics_mixed::<i32, 5>(9);
    assert_eq!(res, "9 5");

    // The fake for (String, 5) should remain unaffected.
    let res = const_generics_mixed::<String, 5>("Test".to_string());
    assert_eq!(res, "Fake Test");
}
