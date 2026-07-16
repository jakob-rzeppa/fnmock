#[fnmock::fakeable]
fn unused_generic<T: 'static, U: 'static>(a: T) -> T {
    a
}

#[test]
fn test_unused_generic() {
    let res = unused_generic::<String, String>("Test".to_string());
    assert_eq!(res, "Test");
}

#[test]
fn test_unused_generic_fake() {
    unused_generic_fake::<String, String>().setup(|a| format!("Fake {}", a));
    let res = unused_generic::<String, String>("Test".to_string());
    assert_eq!(res, "Fake Test");
}
