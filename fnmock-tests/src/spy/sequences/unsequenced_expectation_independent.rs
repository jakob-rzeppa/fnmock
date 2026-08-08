#[fnmock::spyable]
fn independent_fn(id: i32) {
    let _ = id;
}

#[test]
fn test_unsequenced_expectation_is_independent_of_the_sequence() {
    let spy = independent_fn_spy();
    let mut seq = fnmock::Sequence::new();
    spy.expect(fnmock::predicate::eq(2))
        .once()
        .in_sequence(&mut seq);
    spy.expect(fnmock::predicate::eq(3))
        .once()
        .in_sequence(&mut seq);
    // Not in the sequence, so it is not ordered relative to the steps above.
    spy.expect(fnmock::predicate::eq(9)).times(2);

    independent_fn(9); // counted by the unsequenced expectation
    independent_fn(2); // the sequence's current step
    independent_fn(9); // fine, even though the (3) step is still pending
    independent_fn(7); // fine, nothing expects it and nothing complains
    independent_fn(3); // advances the sequence

    spy.assert();
}
