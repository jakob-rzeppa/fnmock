mod spy {
    #[fnmock::spyable]
    fn last_step_fn(id: i32) {
        let _ = id;
    }

    /// Once the sequence reaches its last step, that step stays current: further matching calls
    /// keep being counted against its maximum instead of being ignored.
    #[test]
    fn test_last_step_keeps_counting_matching_calls() {
        let spy = last_step_fn_spy();
        let mut seq = fnmock::Sequence::new();
        spy.expect(fnmock::predicate::eq(2))
            .once()
            .in_sequence(&mut seq);
        spy.expect(fnmock::predicate::eq(7))
            .times(..3)
            .in_sequence(&mut seq);

        last_step_fn(2);
        last_step_fn(7);
        last_step_fn(7); // still counted against the last step, which stays current

        spy.assert();
    }

    /// Because the last step stays current, a call beyond its maximum panics there instead of
    /// being silently dropped.
    #[test]
    #[should_panic(expected = "Too many calls of the spied function")]
    fn test_last_step_panics_once_its_maximum_is_exceeded() {
        let spy = last_step_fn_spy();
        let mut seq = fnmock::Sequence::new();
        spy.expect(fnmock::predicate::eq(2))
            .once()
            .in_sequence(&mut seq);
        spy.expect(fnmock::predicate::eq(7))
            .times(..3)
            .in_sequence(&mut seq);

        last_step_fn(2);
        last_step_fn(7);
        last_step_fn(7);
        last_step_fn(7); // third matching call: over the maximum of `..3`
    }
}

mod mock {
    #[fnmock::mockable]
    fn last_step_fn(id: i32) {
        let _ = id;
    }

    #[test]
    fn test_last_step_keeps_counting_matching_calls() {
        let mock = last_step_fn_mock();
        let mut seq = fnmock::Sequence::new();
        mock.expect(fnmock::predicate::eq(2))
            .once()
            .in_sequence(&mut seq);
        mock.expect(fnmock::predicate::eq(7))
            .times(..3)
            .in_sequence(&mut seq);

        last_step_fn(2);
        last_step_fn(7);
        last_step_fn(7);

        mock.assert();
    }
}
