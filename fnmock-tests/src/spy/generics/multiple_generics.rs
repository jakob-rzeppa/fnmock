#[fnmock::spyable]
fn multiple_generics<T: 'static, U: 'static>(a: T, b: U) -> (T, U) {
    (a, b)
}

#[test]
fn test_multiple_generics() {
    let spy = multiple_generics_spy::<String, i32>();
    spy.expect(
        fnmock::predicate::eq("hi".to_string()),
        fnmock::predicate::eq(2),
    )
    .once();

    let res = multiple_generics("hi".to_string(), 2);

    assert_eq!(res, ("hi".to_string(), 2));
    spy.assert();
}

/// The store is keyed by the generic arguments as a whole, so swapping them
/// around reaches a different instantiation entirely.
#[test]
fn test_swapped_generic_arguments_are_a_different_instantiation() {
    let spy_string_i32 = multiple_generics_spy::<String, i32>();
    let spy_i32_string = multiple_generics_spy::<i32, String>();
    spy_string_i32.expect_once();
    spy_i32_string.expect_never();

    multiple_generics("hi".to_string(), 2);

    spy_string_i32.assert();
    spy_i32_string.assert();
}
