#[fnmock::spyable]
fn lenient_seq_fn(id: i32) {
    let _ = id;
}

/// A call that arrives before the sequence is ready for it is not an error: it is silently
/// dropped and the sequence stays where it is, rather than panicking. Once the real order
/// resumes the sequence still completes normally.
#[test]
fn test_early_call_is_dropped_without_panicking_then_sequence_completes() {
    let spy = lenient_seq_fn_spy();
    let mut seq = fnmock::Sequence::new();
    spy.expect(fnmock::predicate::eq(2))
        .times(3)
        .in_sequence(&mut seq);
    spy.expect(fnmock::predicate::eq(5))
        .once()
        .in_sequence(&mut seq);

    lenient_seq_fn(5); // too early: dropped, the sequence stays on the first step
    lenient_seq_fn(2);
    lenient_seq_fn(2);
    lenient_seq_fn(2);
    lenient_seq_fn(5); // now the first step is fulfilled, so this advances the sequence

    spy.assert();
}

/// If the early call is never repeated at the right point, the sequence itself never panics —
/// the skipped step just never gets its calls, so the failure only shows up at `assert()`.
#[test]
#[should_panic(expected = "Expectation(s) of the spied function")]
fn test_lenient_sequence_fails_assert_if_the_dropped_call_is_never_repeated() {
    let spy = lenient_seq_fn_spy();
    let mut seq = fnmock::Sequence::new();
    spy.expect(fnmock::predicate::eq(2))
        .times(3)
        .in_sequence(&mut seq);
    spy.expect(fnmock::predicate::eq(5))
        .once()
        .in_sequence(&mut seq);

    lenient_seq_fn(2);
    lenient_seq_fn(5); // dropped, not a panic: the sequence is still waiting on (2)
    lenient_seq_fn(2);
    lenient_seq_fn(2);
    // the (5) step never got a call in the right place

    spy.assert();
}
