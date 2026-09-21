#[fnmock::spyable]
fn chain_order_fn(id: i32) {
    let _ = id;
}

/// `in_sequence` may be chained before or after `times`/`once`/`never` — both orders describe
/// the same expectation.
#[test]
fn test_in_sequence_before_or_after_times_is_equivalent() {
    let spy = chain_order_fn_spy();
    let mut seq = fnmock::Sequence::new();
    spy.expect(fnmock::predicate::eq(2))
        .times(2)
        .in_sequence(&mut seq);
    spy.expect(fnmock::predicate::eq(5))
        .in_sequence(&mut seq)
        .once();

    chain_order_fn(2);
    chain_order_fn(2);
    chain_order_fn(5);

    spy.assert();
}
