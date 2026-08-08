#[fnmock::spyable]
fn by_value(value: String) {
    let _ = value;
}

#[test]
fn test_by_value() {
    let spy = by_value_spy();
    spy.expect(fnmock::predicate::eq("hi".to_string()));

    by_value("hi".to_string());

    spy.assert();
}
