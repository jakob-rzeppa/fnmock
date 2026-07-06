#[fnmock::fakeable]
fn bounds_mixed<T: 'static, 'a>(a: &'a mut T) -> &'a T {
    a
}

#[test]
fn test_bounds_mixed() {
    let mut value = "Test".to_string();
    let res = bounds_mixed(&mut value);
    assert_eq!(res, "Test");
}

#[test]
fn test_bounds_mixed_fake() {
    bounds_mixed_fake::<String>().setup(|a| {
        a.push_str(" Fake");
        a
    });
    let mut value = "Test".to_string();
    let res = bounds_mixed(&mut value);
    assert_eq!(res, "Test Fake");
}
