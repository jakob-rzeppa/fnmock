#[fnmock::spyable]
fn by_ref_and_value(id: String, uuid: &str) {
    let _ = (id, uuid);
}

#[test]
fn test_by_ref_and_value() {
    let spy = by_ref_and_value_spy();
    spy.expect(
        fnmock::predicate::eq("hi".to_string()),
        fnmock::predicate::eq("world".to_string()),
    );

    by_ref_and_value("hi".to_string(), "world");

    spy.assert();
}
