#[fnmock::spyable]
fn describe_target(id: i32) {
    let _ = id;
}

#[test]
fn test_describe_sets_custom_name_without_affecting_matching() {
    let spy = describe_target_spy();
    spy.expect(fnmock::predicate::eq(2))
        .describe("first id".to_string())
        .once();

    describe_target(2);

    spy.assert();
}

#[test]
#[should_panic(expected = "first id")]
fn test_describe_message_appears_in_the_panic_when_unfulfilled() {
    let spy = describe_target_spy();
    spy.expect(fnmock::predicate::eq(2))
        .describe("first id".to_string())
        .times(2);

    describe_target(2);

    spy.assert();
}
