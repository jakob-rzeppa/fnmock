#[fnmock::spyable]
fn times_target(id: i32) {
    let _ = id;
}

#[test]
fn test_times_exact() {
    let spy = times_target_spy();
    spy.expect(fnmock::predicate::eq(2)).times(3);

    times_target(2);
    times_target(2);
    times_target(2);

    spy.assert();
}

#[test]
fn test_times_range_inclusive() {
    let spy = times_target_spy();
    spy.expect(fnmock::predicate::eq(2)).times(1..=3);

    times_target(2);
    times_target(2);

    spy.assert();
}

#[test]
fn test_times_range_from() {
    let spy = times_target_spy();
    spy.expect(fnmock::predicate::eq(2)).times(1..);

    for _ in 0..5 {
        times_target(2);
    }

    spy.assert();
}

#[test]
fn test_times_range_to() {
    let spy = times_target_spy();
    spy.expect(fnmock::predicate::eq(2)).times(..3);

    times_target(2);
    times_target(2);

    spy.assert();
}

#[test]
fn test_once() {
    let spy = times_target_spy();
    spy.expect(fnmock::predicate::eq(2)).once();

    times_target(2);

    spy.assert();
}

#[test]
fn test_never() {
    let spy = times_target_spy();
    spy.expect(fnmock::predicate::eq(2)).never();

    times_target(5);

    spy.assert();
}

#[test]
#[should_panic(expected = "Too many calls of the spied function")]
fn test_more_calls_than_times_allows_panics() {
    let spy = times_target_spy();
    spy.expect(fnmock::predicate::eq(2)).times(1);

    times_target(2);
    times_target(2);
}

#[test]
#[should_panic(expected = "Expectation(s) of the spied function")]
fn test_fewer_calls_than_times_requires_fails_assert() {
    let spy = times_target_spy();
    spy.expect(fnmock::predicate::eq(2)).times(2);

    times_target(2);

    spy.assert();
}
