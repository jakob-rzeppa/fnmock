//! `in_sequence` may be chained, putting one expectation into several sequences at once.
//! `ExpectationHandle::drop` appends a *clone* of the expectation to each of them, so every
//! sequence tracks its own call count for that step and orders it against its own other steps.

mod spy {
    #[fnmock::spyable]
    fn shared_step(id: i32) {
        let _ = id;
    }

    #[fnmock::spyable]
    fn preceding_step(id: i32) {
        let _ = id;
    }

    #[test]
    fn test_one_expectation_ordered_by_two_sequences() {
        let shared = shared_step_spy();
        let preceding = preceding_step_spy();
        let mut seq_a = fnmock::Sequence::new();
        let mut seq_b = fnmock::Sequence::new();

        // seq_a is "preceding(1), then shared(0)", seq_b is "preceding(2), then shared(0)".
        preceding
            .expect(fnmock::predicate::eq(1))
            .once()
            .in_sequence(&mut seq_a);
        preceding
            .expect(fnmock::predicate::eq(2))
            .once()
            .in_sequence(&mut seq_b);
        shared
            .expect(fnmock::predicate::eq(0))
            .once()
            .in_sequence(&mut seq_a)
            .in_sequence(&mut seq_b);

        preceding_step(1);
        preceding_step(2);
        shared_step(0);

        shared.assert();
        preceding.assert();
    }

    /// The two clones count independently, so a single call satisfies the step in both sequences —
    /// it is not consumed by whichever sequence sees it first.
    #[test]
    fn test_one_call_satisfies_the_step_in_both_sequences() {
        let shared = shared_step_spy();
        let mut seq_a = fnmock::Sequence::new();
        let mut seq_b = fnmock::Sequence::new();

        shared
            .expect(fnmock::predicate::eq(0))
            .once()
            .in_sequence(&mut seq_a)
            .in_sequence(&mut seq_b);

        shared_step(0);

        shared.assert();
    }

    /// A call placed too early for one of the sequences leaves that sequence's clone at zero
    /// calls, which the assert catches even though the other sequence was happy with it.
    #[test]
    #[should_panic(expected = "Expectation(s) of the spied function")]
    fn test_a_call_too_early_for_one_of_the_two_sequences_fails_assert() {
        let shared = shared_step_spy();
        let preceding = preceding_step_spy();
        let mut seq_a = fnmock::Sequence::new();
        let mut seq_b = fnmock::Sequence::new();

        preceding
            .expect(fnmock::predicate::eq(1))
            .once()
            .in_sequence(&mut seq_a);
        preceding
            .expect(fnmock::predicate::eq(2))
            .once()
            .in_sequence(&mut seq_b);
        shared
            .expect(fnmock::predicate::eq(0))
            .once()
            .in_sequence(&mut seq_a)
            .in_sequence(&mut seq_b);

        preceding_step(1);
        // In order for seq_a, but seq_b is still waiting for preceding(2), so seq_b drops it.
        shared_step(0);
        preceding_step(2);

        shared.assert();
    }
}

mod mock {
    #[fnmock::mockable]
    fn shared_step(id: i32) {
        let _ = id;
    }

    #[fnmock::mockable]
    fn preceding_step(id: i32) {
        let _ = id;
    }

    #[test]
    fn test_one_expectation_ordered_by_two_sequences() {
        let shared = shared_step_mock();
        let preceding = preceding_step_mock();
        let mut seq_a = fnmock::Sequence::new();
        let mut seq_b = fnmock::Sequence::new();

        preceding
            .expect(fnmock::predicate::eq(1))
            .once()
            .in_sequence(&mut seq_a);
        preceding
            .expect(fnmock::predicate::eq(2))
            .once()
            .in_sequence(&mut seq_b);
        shared
            .expect(fnmock::predicate::eq(0))
            .once()
            .in_sequence(&mut seq_a)
            .in_sequence(&mut seq_b);

        preceding_step(1);
        preceding_step(2);
        shared_step(0);

        shared.assert();
        preceding.assert();
    }
}
