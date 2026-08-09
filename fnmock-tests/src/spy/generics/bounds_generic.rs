//! Inline trait bounds. They have to survive onto the generated matcher and
//! interface, which repeat the function's generic parameters.

#[fnmock::spyable]
fn bounds_generic<T: Clone + std::fmt::Debug + 'static>(a: T) -> T {
    a.clone()
}

#[test]
fn test_bounds_generic() {
    let spy = bounds_generic_spy::<String>();
    spy.expect(fnmock::predicate::eq("hi".to_string())).once();

    let res = bounds_generic("hi".to_string());

    assert_eq!(res, "hi");
    spy.assert();
}
