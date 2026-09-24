//! A mock instantiation controls the fake *and* the expectations of that instantiation only.

#[fnmock::mockable]
fn mock_generics<T: 'static>(a: T) -> T {
    a
}

#[test]
fn test_instantiations_are_independent_on_both_halves() {
    let mock_i32 = mock_generics_mock::<i32>();
    let mock_u8 = mock_generics_mock::<u8>();
    mock_i32.setup(|a| a + 100);
    mock_i32.expect(fnmock::predicate::eq(1)).once();
    mock_u8.expect(fnmock::predicate::eq(1)).once();

    assert_eq!(mock_generics(1i32), 101);
    assert_eq!(mock_generics(1u8), 1);

    mock_i32.assert();
    mock_u8.assert();
}

#[test]
fn test_call_of_one_instantiation_does_not_satisfy_another() {
    let mock_i32 = mock_generics_mock::<i32>();
    let mock_u8 = mock_generics_mock::<u8>();
    mock_i32.setup(|a| a);
    mock_i32.expect_once();
    mock_u8.expect_once();

    mock_generics(1i32);

    mock_i32.assert();
}

#[test]
#[should_panic(expected = "Expectation(s) of the spied function")]
fn test_unfulfilled_expectation_of_another_instantiation_fails_its_own_assert() {
    let mock_i32 = mock_generics_mock::<i32>();
    let mock_u8 = mock_generics_mock::<u8>();
    mock_i32.expect_once();
    mock_u8.expect_once();

    mock_generics(1i32);

    mock_u8.assert();
}

#[test]
fn test_is_set_is_per_instantiation() {
    mock_generics_mock::<i32>().setup(|a| a);

    assert!(mock_generics_mock::<i32>().is_set());
    assert!(!mock_generics_mock::<u8>().is_set());
}
