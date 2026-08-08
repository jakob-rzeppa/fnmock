#[fnmock::spyable]
fn function_executes_normally(value: i32) -> i32 {
    value + 1
}

#[test]
fn test_function_executes_normally() {
    let spy = function_executes_normally_spy();
    spy.expect(fnmock::predicate::eq(1));

    let result = function_executes_normally(1);

    assert_eq!(result, 2);
    spy.assert();
}
