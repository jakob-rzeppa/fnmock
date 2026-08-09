//! A type parameter and a const parameter on the same function. The key holds
//! a `TypeId` for the first and the actual value for the second.

#[fnmock::spyable]
fn const_generics_mixed<T: 'static, const C: usize>(a: T) -> usize {
    let _ = a;
    C
}

#[test]
fn test_const_generics_mixed() {
    let spy = const_generics_mixed_spy::<String, 4>();
    spy.expect(fnmock::predicate::eq("hi".to_string())).once();

    let res = const_generics_mixed::<String, 4>("hi".to_string());

    assert_eq!(res, 4);
    spy.assert();
}

#[test]
fn test_const_generics_mixed_isolation() {
    let spy_string_4 = const_generics_mixed_spy::<String, 4>();
    let spy_string_8 = const_generics_mixed_spy::<String, 8>();
    let spy_i32_4 = const_generics_mixed_spy::<i32, 4>();
    spy_string_4.expect_once();
    spy_string_8.expect_never();
    spy_i32_4.expect_never();

    const_generics_mixed::<String, 4>("hi".to_string());

    spy_string_4.assert();
    spy_string_8.assert();
    spy_i32_4.assert();
}
