#[fnmock::fakeable]
fn cross_type_isolation<T: 'static>(a: T) -> T {
    a
}

#[test]
fn test_cross_type_isolation() {
    let res = cross_type_isolation("Test".to_string());
    assert_eq!(res, "Test");

    let res = cross_type_isolation(42);
    assert_eq!(res, 42);
}

#[test]
fn test_cross_type_isolation_fake() {
    cross_type_isolation_fake::<String>().setup(|a| format!("Fake {}", a));

    let res = cross_type_isolation("Test".to_string());
    assert_eq!(res, "Fake Test");

    // The fake for String should not affect the behavior for i32.
    let res = cross_type_isolation(42);
    assert_eq!(res, 42);

    cross_type_isolation_fake::<i32>().setup(|a| a + 1);

    let res = cross_type_isolation(42);
    assert_eq!(res, 43);
}
