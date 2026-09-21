//! `f_spy::<T>().assert()` checks only the `T` instantiation. This is the
//! reason `f_spy_all()` exists, so it is worth pinning down directly.

#[fnmock::spyable]
fn assert_scoped_to_instantiation<T: 'static>(a: T) {
    let _ = a;
}

#[test]
fn test_assert_ignores_another_instantiations_unfulfilled_expectation() {
    let spy_string = assert_scoped_to_instantiation_spy::<String>();
    let spy_i32 = assert_scoped_to_instantiation_spy::<i32>();

    spy_string.expect(fnmock::predicate::always()).once();
    // Never fulfilled, and deliberately never asserted.
    spy_i32.expect(fnmock::predicate::always()).once();

    assert_scoped_to_instantiation("hi".to_string());

    // Passes despite the i32 instantiation being unsatisfied.
    spy_string.assert();
}

#[test]
#[should_panic(expected = "Expectation(s) of the spied function")]
fn test_assert_does_catch_its_own_instantiation() {
    let spy_i32 = assert_scoped_to_instantiation_spy::<i32>();
    spy_i32.expect(fnmock::predicate::always()).once();

    spy_i32.assert();
}
