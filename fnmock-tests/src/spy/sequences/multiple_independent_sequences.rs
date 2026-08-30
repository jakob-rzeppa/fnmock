#[fnmock::spyable]
fn multi_seq_fn(id: i32) {
    let _ = id;
}

#[test]
fn test_two_sequences_on_the_same_spy_progress_independently() {
    let spy = multi_seq_fn_spy();
    let mut seq_a = fnmock::Sequence::new();
    let mut seq_b = fnmock::Sequence::new();
    spy.expect(fnmock::predicate::eq(1))
        .once()
        .in_sequence(&mut seq_a);
    spy.expect(fnmock::predicate::eq(2))
        .once()
        .in_sequence(&mut seq_a);
    spy.expect(fnmock::predicate::eq(3))
        .once()
        .in_sequence(&mut seq_b);
    spy.expect(fnmock::predicate::eq(4))
        .once()
        .in_sequence(&mut seq_b);

    multi_seq_fn(1); // seq_a step 1
    multi_seq_fn(3); // seq_b step 1
    multi_seq_fn(2); // seq_a step 2
    multi_seq_fn(4); // seq_b step 2

    spy.assert();
}
