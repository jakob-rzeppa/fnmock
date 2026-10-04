//! `Sequence::default()` has to be the same thing as `Sequence::new()` — not `Sequence::strict()`.
//! This test ensures that the default sequence is lenient, not strict.

mod spy {
    #[fnmock::spyable]
    fn default_seq_target(id: i32) {
        let _ = id;
    }

    #[test]
    fn test_default_sequence_is_lenient_not_strict() {
        let spy = default_seq_target_spy();
        let seq = fnmock::Sequence::default();
        spy.expect(fnmock::predicate::eq(1))
            .once()
            .in_sequence(&seq);
        spy.expect(fnmock::predicate::eq(2))
            .once()
            .in_sequence(&seq);

        default_seq_target(2);
        default_seq_target(1);
        default_seq_target(2);

        spy.assert();
    }
}

mod mock {
    #[fnmock::mockable]
    fn default_seq_target(id: i32) {
        let _ = id;
    }

    #[test]
    fn test_default_sequence_is_lenient_not_strict() {
        let mock = default_seq_target_mock();
        let seq = fnmock::Sequence::default();
        mock.expect(fnmock::predicate::eq(1))
            .once()
            .in_sequence(&seq);
        mock.expect(fnmock::predicate::eq(2))
            .once()
            .in_sequence(&seq);

        default_seq_target(2);
        default_seq_target(1);
        default_seq_target(2);

        mock.assert();
    }
}
