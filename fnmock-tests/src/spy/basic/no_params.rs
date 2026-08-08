#[fnmock::spyable]
fn no_params() {}

#[test]
fn test_no_params() {
    let spy = no_params_spy();
    spy.expect_times(1);
    spy.expect();

    no_params();

    spy.assert();
}
