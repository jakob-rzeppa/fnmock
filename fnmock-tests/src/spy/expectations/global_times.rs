mod spy {
    #[fnmock::spyable]
    fn global_times_target(id: i32) {
        let _ = id;
    }

    #[test]
    fn test_expect_times() {
        let spy = global_times_target_spy();
        spy.expect_times(2);

        global_times_target(1);
        global_times_target(2);

        spy.assert();
    }

    #[test]
    fn test_expect_once() {
        let spy = global_times_target_spy();
        spy.expect_once();

        global_times_target(1);

        spy.assert();
    }

    #[test]
    fn test_expect_never() {
        let spy = global_times_target_spy();
        spy.expect_never();

        spy.assert();
    }

    #[test]
    #[should_panic(expected = "got 1, expected exactly 0")]
    fn test_more_calls_than_never() {
        let spy = global_times_target_spy();
        spy.expect_never();

        global_times_target(2);

        spy.assert();
    }

    #[test]
    fn test_expect_times_range() {
        let spy = global_times_target_spy();
        spy.expect_times(2..);

        global_times_target(1);
        global_times_target(2);
        global_times_target(3);

        spy.assert();
    }

    #[test]
    fn test_global_times_independent_of_predicate_expectation() {
        let spy = global_times_target_spy();
        spy.expect_times(2);
        spy.expect(fnmock::predicate::eq(2)).once();

        global_times_target(2);
        global_times_target(5);

        spy.assert();
    }

    #[test]
    #[should_panic(expected = "got 2, expected exactly 1")]
    fn test_more_calls_than_expect_times_allows_panics() {
        let spy = global_times_target_spy();
        spy.expect_times(1);

        global_times_target(1);
        global_times_target(2);
    }

    #[test]
    #[should_panic(expected = "Expectation(s) of the spied function")]
    fn test_fewer_calls_than_expect_times_requires_fails_assert() {
        let spy = global_times_target_spy();
        spy.expect_times(2);

        global_times_target(1);

        spy.assert();
    }

    #[test]
    fn test_global_times_does_not_affect_a_sequence() {
        let spy = global_times_target_spy();
        let mut seq = fnmock::Sequence::new();
        spy.expect_times(3); // counts every call, regardless of the sequence's progress
        spy.expect(fnmock::predicate::eq(2))
            .once()
            .in_sequence(&mut seq);
        spy.expect(fnmock::predicate::eq(5))
            .once()
            .in_sequence(&mut seq);

        global_times_target(2);
        global_times_target(9); // matches no argument expectation, but still counted globally
        global_times_target(5);

        spy.assert();
    }
}

mod mock {
    #[fnmock::mockable]
    fn global_times_target(id: i32) {
        let _ = id;
    }

    #[test]
    fn test_expect_times() {
        let mock = global_times_target_mock();
        mock.expect_times(2);

        global_times_target(1);
        global_times_target(2);

        mock.assert();
    }

    #[test]
    fn test_setup_and_expect_times_together() {
        let mock = global_times_target_mock();
        mock.setup(|_id| ());
        mock.expect_times(2);

        global_times_target(1);
        global_times_target(2);

        mock.assert();
    }
}
