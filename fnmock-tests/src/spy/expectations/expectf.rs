#[fnmock::spyable]
fn expectf_target(id: i32) {
    let _ = id;
}

#[test]
fn test_expectf_default() {
    let spy = expectf_target_spy();
    spy.expectf(|id: &i32| *id == 2);

    expectf_target(2);

    spy.assert();
}

#[test]
fn test_expectf_times() {
    let spy = expectf_target_spy();
    spy.expectf(|id: &i32| *id == 2).times(2);

    expectf_target(2);
    expectf_target(2);

    spy.assert();
}
