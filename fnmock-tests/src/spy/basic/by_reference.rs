#[fnmock::spyable]
fn by_reference(value: &str) {
    let _ = value;
}

#[test]
fn test_by_reference() {
    let spy = by_reference_spy();
    spy.expect(fnmock::predicate::eq("hi".to_string()));

    by_reference("hi");

    spy.assert();
}
