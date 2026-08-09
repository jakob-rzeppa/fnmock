#[fnmock::spyable]
fn single_generic<T: 'static>(a: T) -> T {
    a
}

#[test]
fn test_single_generic() {
    let spy = single_generic_spy::<String>();
    spy.expect(fnmock::predicate::eq("hi".to_string())).once();

    let res = single_generic("hi".to_string());

    assert_eq!(res, "hi");
    spy.assert();
}

#[test]
fn test_spied_generic_function_still_returns_its_real_value() {
    let spy = single_generic_spy::<i32>();
    spy.expect_times(2);

    assert_eq!(single_generic(42), 42);
    assert_eq!(single_generic(-1), -1);

    spy.assert();
}

#[test]
#[should_panic(expected = "Expectation(s) of the spied function")]
fn test_unfulfilled_expectation_on_an_instantiation_fails_assert() {
    let spy = single_generic_spy::<u8>();
    spy.expect(fnmock::predicate::eq(1u8)).once();

    spy.assert();
}
