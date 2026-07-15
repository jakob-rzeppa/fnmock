use std::fmt::Display;

#[fnmock::fakeable]
fn bounds_generic<T: 'static, U: 'static + Clone>(a: (T, U)) -> (T, U) {
    a
}

#[test]
fn test_bounds_generic() {
    let res = bounds_generic(("Test".to_string(), "Another".to_string()));
    assert_eq!(res, ("Test".to_string(), "Another".to_string()));
}

#[test]
fn test_bounds_generic_fake() {
    // You should always specify the generic types when setting up a fake for a function with generics.
    // It is possible for the compiler to infer the types, but it is not guaranteed to work in all cases
    // and makes the code less readable.
    bounds_generic_fake::<String, String>().setup(|a| (
        format!("Fake {}", a.0),
        format!("Fake {}", a.1),
    ));
    let res = bounds_generic(("Test".to_string(), "Another".to_string()));
    assert_eq!(res, ("Fake Test".to_string(), "Fake Another".to_string()));
}

#[fnmock::fakeable]
fn bounds_generic_where<T, U>(a: (T, U)) -> (T, U) where T: 'static, U: 'static + Clone {
    a
}

#[test]
fn test_bounds_generic_where() {
    let res = bounds_generic_where(("Test".to_string(), "Another".to_string()));
    assert_eq!(res, ("Test".to_string(), "Another".to_string()));
}

#[test]
fn test_bounds_generic_where_fake() {
    // You should always specify the generic types when setting up a fake for a function with generics.
    // It is possible for the compiler to infer the types, but it is not guaranteed to work in all cases
    // and makes the code less readable.
    bounds_generic_where_fake::<String, String>().setup(|a| (
        format!("Fake {}", a.0),
        format!("Fake {}", a.1),
    ));
    let res = bounds_generic_where(("Test".to_string(), "Another".to_string()));
    assert_eq!(res, ("Fake Test".to_string(), "Fake Another".to_string()));
}

#[fnmock::fakeable]
fn bounds_generic_inline_and_where<T, U: 'static + Display>(a: (T, U)) -> (T, U)
    where T: 'static, U: Clone
{
    a
}

#[test]
fn test_bounds_generic_inline_and_where() {
    let res = bounds_generic_inline_and_where(("Test".to_string(), "Another".to_string()));
    assert_eq!(res, ("Test".to_string(), "Another".to_string()));
}

#[test]
fn test_bounds_generic_inline_and_where_fake() {
    // You should always specify the generic types when setting up a fake for a function with generics.
    // It is possible for the compiler to infer the types, but it is not guaranteed to work in all cases
    // and makes the code less readable.
    bounds_generic_inline_and_where_fake::<String, String>().setup(|a| (
        format!("Fake {}", a.0),
        format!("Fake {}", a.1),
    ));
    let res = bounds_generic_inline_and_where(("Test".to_string(), "Another".to_string()));
    assert_eq!(res, ("Fake Test".to_string(), "Fake Another".to_string()));
}
