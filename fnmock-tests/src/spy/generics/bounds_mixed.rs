//! One parameter bounded inline, another in a `where` clause, on the same
//! function.

#[fnmock::spyable]
fn bounds_mixed<T: Clone + 'static, U>(a: T, b: U) -> (T, U)
where
    U: std::fmt::Debug + 'static,
{
    (a, b)
}

#[test]
fn test_bounds_mixed() {
    let spy = bounds_mixed_spy::<String, i32>();
    spy.expect(
        fnmock::predicate::eq("hi".to_string()),
        fnmock::predicate::eq(2),
    )
    .once();

    let res = bounds_mixed("hi".to_string(), 2);

    assert_eq!(res, ("hi".to_string(), 2));
    spy.assert();
}
