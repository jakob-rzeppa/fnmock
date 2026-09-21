#[fnmock::spyable]
fn strict_seq_fn(id: i32) {
    let _ = id;
}

#[test]
fn test_strict_sequence_calls_in_order_pass() {
    let spy = strict_seq_fn_spy();
    let mut seq = fnmock::Sequence::new_strict();
    spy.expect(fnmock::predicate::eq(2))
        .times(2)
        .in_sequence(&mut seq);
    spy.expect(fnmock::predicate::eq(5))
        .once()
        .in_sequence(&mut seq);

    strict_seq_fn(2);
    strict_seq_fn(2);
    strict_seq_fn(5);

    spy.assert();
}

#[test]
#[should_panic(expected = "Call out of sequence")]
fn test_strict_sequence_panics_immediately_on_out_of_order_call() {
    let spy = strict_seq_fn_spy();
    let mut seq = fnmock::Sequence::new_strict();
    spy.expect(fnmock::predicate::eq(2))
        .times(3)
        .in_sequence(&mut seq);
    spy.expect(fnmock::predicate::eq(5))
        .once()
        .in_sequence(&mut seq);

    strict_seq_fn(2);
    strict_seq_fn(5); // too early: the (2) step hasn't reached its minimum of 3 yet
}
