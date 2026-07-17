#[fnmock::fakeable]
fn multiple_generics<T: 'static, U: 'static>(a: (T, U)) -> (T, U) {
    a
}

#[test]
fn test_multiple_generics() {
    let res = multiple_generics(("Test".to_string(), "Another".to_string()));
    assert_eq!(res, ("Test".to_string(), "Another".to_string()));
}

#[test]
fn test_multiple_generics_fake() {
    // You should always specify the generic types when setting up a fake for a function with generics.
    // It is possible for the compiler to infer the types, but it is not guaranteed to work in all cases
    // and makes the code less readable.
    multiple_generics_fake::<String, String>()
        .setup(|a| (format!("Fake {}", a.0), format!("Fake {}", a.1)));
    let res = multiple_generics(("Test".to_string(), "Another".to_string()));
    assert_eq!(res, ("Fake Test".to_string(), "Fake Another".to_string()));
}
