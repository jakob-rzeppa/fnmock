//! The same bounds as `bounds_generic`, expressed in a `where` clause. The
//! macro folds them back into inline bounds before repeating them.

#[fnmock::spyable]
fn bounds_where<T>(a: T) -> T
where
    T: Clone + std::fmt::Debug + 'static,
{
    a.clone()
}

#[test]
fn test_bounds_where() {
    let spy = bounds_where_spy::<String>();
    spy.expect(fnmock::predicate::eq("hi".to_string())).once();

    let res = bounds_where("hi".to_string());

    assert_eq!(res, "hi");
    spy.assert();
}
