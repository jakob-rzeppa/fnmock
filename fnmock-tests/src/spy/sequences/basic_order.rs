#[fnmock::spyable]
fn sequenced_fn(id: i32) {
    let _ = id;
}

#[test]
fn test_calls_in_order_fulfill_sequence() {
    let spy = sequenced_fn_spy();
    let mut seq = fnmock::Sequence::new();
    spy.expect(fnmock::predicate::eq(2))
        .times(3)
        .in_sequence(&mut seq);
    spy.expect(fnmock::predicate::eq(5))
        .once()
        .in_sequence(&mut seq);

    sequenced_fn(2);
    sequenced_fn(2);
    sequenced_fn(2);
    sequenced_fn(5);

    spy.assert();
}
