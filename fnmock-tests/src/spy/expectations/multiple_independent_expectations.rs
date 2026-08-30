#[fnmock::spyable]
fn multi_target(id: i32) {
    let _ = id;
}

#[test]
fn test_multiple_expectations_fulfilled_independently() {
    let spy = multi_target_spy();
    spy.expect(fnmock::predicate::eq(2)).times(3);
    spy.expect(fnmock::predicate::eq(5)).times(1..=3);

    multi_target(2);
    multi_target(2);
    multi_target(2);
    multi_target(5);

    spy.assert();
}
