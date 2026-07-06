#[fnmock::fakeable]
fn return_generic<T: 'static>(a: T) -> T {
    a
}

#[test]
fn test_return_generic() {
    let res = return_generic("Test".to_string());
    assert_eq!(res, "Test");
}

#[test]
fn test_return_generic_fake() {
    return_generic_fake::<String>().setup(|a| format!("Fake {}", a));
    let res = return_generic("Test".to_string());
    assert_eq!(res, "Fake Test");
}
