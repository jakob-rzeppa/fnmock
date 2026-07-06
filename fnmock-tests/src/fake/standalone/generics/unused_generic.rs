#[fnmock::fakeable]
fn single_generic<T: 'static, U: 'static>(a: T) -> T {
    a
}

#[test]
fn test_single_generic() {
    let res = single_generic::<String, String>("Test".to_string());
    assert_eq!(res, "Test");
}

#[test]
fn test_single_generic_fake() {
    single_generic_fake::<String, String>().setup(|a| format!("Fake {}", a));
    let res = single_generic::<String, String>("Test".to_string());
    assert_eq!(res, "Fake Test");
}
