#[fnmock::spyable]
fn unexpected_call_target(id: i32) {
    let _ = id;
}

#[test]
fn test_call_matching_no_expectation_is_ignored() {
    let spy = unexpected_call_target_spy();
    spy.expect(fnmock::predicate::eq(2)).once();

    unexpected_call_target(2);
    unexpected_call_target(9);

    spy.assert();
}
