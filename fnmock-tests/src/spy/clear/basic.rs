#[fnmock::spyable]
fn clear_basic(id: i32) {
    let _ = id;
}

#[test]
fn test_clear_drops_unfulfilled_expectations() {
    let spy = clear_basic_spy();
    spy.expect(fnmock::predicate::eq(1)).once();

    spy.clear();

    spy.assert();
}

#[test]
fn test_clear_drops_global_times() {
    let spy = clear_basic_spy();
    spy.expect_times(3);

    spy.clear();

    spy.assert();
}

#[test]
fn test_clear_drops_expectations_that_would_reject_calls() {
    let spy = clear_basic_spy();
    spy.expect(fnmock::predicate::eq(1)).never();
    spy.expect_never();

    spy.clear();

    clear_basic(1);
}

#[test]
fn test_clear_resets_call_history() {
    let spy = clear_basic_spy();
    spy.expect(fnmock::predicate::eq(1)).times(2);
    clear_basic(1);
    clear_basic(1);

    spy.clear();

    // Would panic with "Too many calls" if the earlier calls were still counted.
    spy.expect(fnmock::predicate::eq(1)).once();
    clear_basic(1);
    spy.assert();
}

#[test]
fn test_clear_resets_global_call_count() {
    let spy = clear_basic_spy();
    spy.expect_once();
    clear_basic(1);

    spy.clear();

    spy.expect_once();
    clear_basic(2);
    spy.assert();
}

#[test]
fn test_expectations_set_after_clear_are_enforced() {
    let spy = clear_basic_spy();
    spy.expect(fnmock::predicate::eq(1)).once();
    spy.clear();

    spy.expect(fnmock::predicate::eq(2)).once();
    clear_basic(2);

    spy.assert();
}

#[test]
#[should_panic(expected = "Expectation(s) of the spied function")]
fn test_expectation_set_after_clear_can_still_fail() {
    let spy = clear_basic_spy();
    spy.expect(fnmock::predicate::eq(1)).once();
    spy.clear();

    spy.expect(fnmock::predicate::eq(2)).once();

    spy.assert();
}

#[test]
fn test_clear_without_expectations_is_a_noop() {
    let spy = clear_basic_spy();

    spy.clear();
    spy.clear();

    spy.assert();
}
