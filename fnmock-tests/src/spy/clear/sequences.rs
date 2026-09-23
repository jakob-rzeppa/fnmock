#[fnmock::spyable]
fn clear_seq_first(id: i32) {
    let _ = id;
}

#[fnmock::spyable]
fn clear_seq_second(id: i32) {
    let _ = id;
}

#[test]
fn test_clear_removes_the_spys_steps_from_a_sequence() {
    let first = clear_seq_first_spy();
    let second = clear_seq_second_spy();
    let mut seq = fnmock::Sequence::new_strict();
    first
        .expect(fnmock::predicate::always())
        .once()
        .in_sequence(&mut seq);
    second
        .expect(fnmock::predicate::always())
        .once()
        .in_sequence(&mut seq);

    first.clear();

    // `second` is now the first step; without the clear this is out of order.
    clear_seq_second(1);

    second.assert();
    first.assert();
}

#[test]
#[should_panic(expected = "Expectation(s) of the spied function")]
fn test_clear_leaves_other_functions_steps_in_the_sequence() {
    let first = clear_seq_first_spy();
    let second = clear_seq_second_spy();
    let mut seq = fnmock::Sequence::new();
    first
        .expect(fnmock::predicate::always())
        .once()
        .in_sequence(&mut seq);
    second
        .expect(fnmock::predicate::always())
        .once()
        .in_sequence(&mut seq);

    first.clear();

    // `second` is never called, so its step must still be reported.
    second.assert();
}

#[test]
fn test_cleared_spy_can_join_a_new_sequence() {
    let first = clear_seq_first_spy();
    let second = clear_seq_second_spy();
    let mut old_seq = fnmock::Sequence::new_strict();
    first
        .expect(fnmock::predicate::always())
        .once()
        .in_sequence(&mut old_seq);
    first.clear();

    let mut new_seq = fnmock::Sequence::new_strict();
    second
        .expect(fnmock::predicate::always())
        .once()
        .in_sequence(&mut new_seq);
    first
        .expect(fnmock::predicate::always())
        .once()
        .in_sequence(&mut new_seq);

    clear_seq_second(1);
    clear_seq_first(1);

    first.assert();
    second.assert();
}
