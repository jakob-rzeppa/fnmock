//! On a mock, `clear()` resets both halves: the installed fake, and every expectation, global
//! `expect_times` range and recorded call.

#[fnmock::mockable]
fn clear_mock_basic(id: i32) -> i32 {
    id
}

#[fnmock::mockable]
fn clear_mock_generic<T: 'static>(a: T) -> T {
    a
}

#[test]
fn test_clear_removes_the_fake() {
    let mock = clear_mock_basic_mock();
    mock.setup(|id| id + 100);
    assert_eq!(clear_mock_basic(1), 101);

    mock.clear();

    assert!(!mock.is_set());
    assert_eq!(clear_mock_basic(1), 1);
}

#[test]
fn test_clear_drops_unfulfilled_expectations() {
    let mock = clear_mock_basic_mock();
    mock.expect(fnmock::predicate::eq(1)).once();
    mock.expect_times(3);

    mock.clear();

    mock.assert();
}

#[test]
fn test_clear_drops_expectations_that_would_reject_calls() {
    let mock = clear_mock_basic_mock();
    mock.expect(fnmock::predicate::eq(1)).never();
    mock.expect_never();

    mock.clear();

    clear_mock_basic(1);
    mock.assert();
}

#[test]
fn test_clear_resets_both_halves_at_once() {
    let mock = clear_mock_basic_mock();
    mock.setup(|id| id + 100);
    mock.expect(fnmock::predicate::eq(1)).once();
    assert_eq!(clear_mock_basic(1), 101);

    mock.clear();

    // Fake gone: the real body runs. Expectation and call history gone: a fresh `once` holds.
    mock.expect(fnmock::predicate::eq(2)).once();
    assert_eq!(clear_mock_basic(2), 2);
    mock.assert();
}

#[test]
fn test_cleared_mock_can_be_set_up_again() {
    let mock = clear_mock_basic_mock();
    mock.setup(|id| id + 1);
    mock.clear();

    mock.setup(|id| id + 2);
    mock.expect(fnmock::predicate::eq(5)).once();

    assert_eq!(clear_mock_basic(5), 7);
    mock.assert();
}

#[test]
fn test_clear_resets_call_history() {
    let mock = clear_mock_basic_mock();
    mock.expect(fnmock::predicate::eq(1)).times(2);
    clear_mock_basic(1);
    clear_mock_basic(1);

    mock.clear();

    // Would panic with "Too many calls" if the earlier calls were still counted.
    mock.expect(fnmock::predicate::eq(1)).once();
    clear_mock_basic(1);
    mock.assert();
}

#[test]
fn test_clear_inside_the_fake_closure_does_not_panic() {
    clear_mock_basic_mock().setup(|id| {
        clear_mock_basic_mock().clear();
        id + 100
    });

    // The in-flight call finishes with the closure that was installed when it started...
    assert_eq!(clear_mock_basic(1), 101);
    // ...but it cleared itself, so the next call runs the real body.
    assert_eq!(clear_mock_basic(1), 1);
}

#[test]
fn test_generic_clear_resets_both_halves_of_one_instantiation_only() {
    let mock_i32 = clear_mock_generic_mock::<i32>();
    let mock_u8 = clear_mock_generic_mock::<u8>();
    mock_i32.setup(|a| a + 1);
    mock_u8.setup(|a| a + 1);
    mock_i32.expect_once();
    mock_u8.expect_once();

    mock_i32.clear();

    assert!(!mock_i32.is_set());
    assert!(mock_u8.is_set());
    mock_i32.assert();
    assert_eq!(clear_mock_generic(1u8), 2);
    mock_u8.assert();
}
