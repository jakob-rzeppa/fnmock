#[fnmock::spyable]
fn never_step_fn(id: i32) {
    let _ = id;
}

/// A `.never()` step has minimum 0, so it is advancable immediately and the sequence can skip
/// over it without it ever being called.
#[test]
fn test_never_step_is_skipped_when_not_called() {
    let spy = never_step_fn_spy();
    let mut seq = fnmock::Sequence::new();
    spy.expect(fnmock::predicate::eq(2))
        .once()
        .in_sequence(&mut seq);
    spy.expect(fnmock::predicate::eq(4))
        .never()
        .in_sequence(&mut seq);
    spy.expect(fnmock::predicate::eq(5))
        .once()
        .in_sequence(&mut seq);

    never_step_fn(2);
    never_step_fn(5); // advances straight past the never() step, which is already advancable

    spy.assert();
}

/// If a call does arrive while the `.never()` step is current, it is over its maximum and
/// panics just like any other step would.
#[test]
#[should_panic(expected = "Too many calls of the spied function")]
fn test_never_step_panics_if_called_while_current() {
    let spy = never_step_fn_spy();
    let mut seq = fnmock::Sequence::new();
    spy.expect(fnmock::predicate::eq(2))
        .once()
        .in_sequence(&mut seq);
    spy.expect(fnmock::predicate::eq(4))
        .never()
        .in_sequence(&mut seq);

    never_step_fn(2);
    never_step_fn(4); // the never() step is current and matches: over its maximum of 0
}
