//! A mock records every call, whether or not the fake intercepts it, and an intercepted call never
//! runs the real body.

use std::cell::Cell;

thread_local! {
    static REAL_BODY_RUNS: Cell<u32> = const { Cell::new(0) };
}

#[fnmock::mockable]
fn record_then_fake(id: i32) -> i32 {
    REAL_BODY_RUNS.with(|runs| runs.set(runs.get() + 1));
    id
}

fn real_body_runs() -> u32 {
    REAL_BODY_RUNS.with(Cell::get)
}

#[test]
fn test_faked_call_is_recorded_and_matches_expectation() {
    let mock = record_then_fake_mock();
    mock.setup(|id| id * 10);
    mock.expect(fnmock::predicate::eq(4)).once();

    assert_eq!(record_then_fake(4), 40);

    mock.assert();
}

#[test]
fn test_faked_call_skips_real_body() {
    let mock = record_then_fake_mock();
    mock.setup(|id| id * 10);
    mock.expect_times(2);

    record_then_fake(1);
    record_then_fake(2);

    assert_eq!(real_body_runs(), 0);
    mock.assert();
}

#[test]
fn test_unfaked_call_runs_real_body_and_is_recorded() {
    let mock = record_then_fake_mock();
    mock.expect(fnmock::predicate::eq(7)).once();

    assert_eq!(record_then_fake(7), 7);

    assert_eq!(real_body_runs(), 1);
    mock.assert();
}

#[test]
#[should_panic(expected = "Expectation(s) of the spied function")]
fn test_faked_call_that_misses_the_expectation_fails_assert() {
    let mock = record_then_fake_mock();
    mock.setup(|id| id * 10);
    mock.expect(fnmock::predicate::eq(1)).once();

    record_then_fake(2);

    mock.assert();
}

#[test]
fn test_fake_receives_the_arguments_the_expectation_matched() {
    let mock = record_then_fake_mock();
    mock.setup(|id| id + 1);
    mock.expect(fnmock::predicate::eq(5)).times(2);

    assert_eq!(record_then_fake(5), 6);
    assert_eq!(record_then_fake(5), 6);

    mock.assert();
}
