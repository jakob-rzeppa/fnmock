mod spy {
    #[fnmock::spyable]
    fn times_target(id: i32) {
        let _ = id;
    }

    #[test]
    fn test_times_exact() {
        let spy = times_target_spy();
        spy.expect(fnmock::predicate::eq(2)).times(3);

        times_target(2);
        times_target(2);
        times_target(2);

        spy.assert();
    }

    #[test]
    fn test_times_range_inclusive() {
        let spy = times_target_spy();
        spy.expect(fnmock::predicate::eq(2)).times(1..=3);

        times_target(2);
        times_target(2);

        spy.assert();
    }

    #[test]
    fn test_times_range_from() {
        let spy = times_target_spy();
        spy.expect(fnmock::predicate::eq(2)).times(1..);

        for _ in 0..5 {
            times_target(2);
        }

        spy.assert();
    }

    #[test]
    fn test_times_range_to() {
        let spy = times_target_spy();
        spy.expect(fnmock::predicate::eq(2)).times(..3);

        times_target(2);
        times_target(2);

        spy.assert();
    }

    #[test]
    fn test_once() {
        let spy = times_target_spy();
        spy.expect(fnmock::predicate::eq(2)).once();

        times_target(2);

        spy.assert();
    }

    #[test]
    fn test_never() {
        let spy = times_target_spy();
        spy.expect(fnmock::predicate::eq(2)).never();

        times_target(5);

        spy.assert();
    }

    #[test]
    #[should_panic(expected = "Too many calls of the spied function")]
    fn test_more_calls_than_times_allows_panics() {
        let spy = times_target_spy();
        spy.expect(fnmock::predicate::eq(2)).times(1);

        times_target(2);
        times_target(2);
    }

    #[test]
    #[should_panic(expected = "Expectation(s) of the spied function")]
    fn test_fewer_calls_than_times_requires_fails_assert() {
        let spy = times_target_spy();
        spy.expect(fnmock::predicate::eq(2)).times(2);

        times_target(2);

        spy.assert();
    }

    #[test]
    fn test_times_range_includes_its_start() {
        let spy = times_target_spy();
        spy.expect(fnmock::predicate::eq(2)).times(2..5);

        times_target(2);
        times_target(2);

        spy.assert();
    }

    #[test]
    #[should_panic(expected = "Too many calls of the spied function")]
    fn test_times_range_panics_at_its_exclusive_end() {
        let spy = times_target_spy();
        spy.expect(fnmock::predicate::eq(2)).times(2..4);

        times_target(2);
        times_target(2);
        times_target(2);
        times_target(2);
    }

    #[test]
    fn test_times_range_full_accepts_any_number_of_calls() {
        let spy = times_target_spy();
        spy.expect(fnmock::predicate::eq(2)).times(..);

        // Unbounded on both ends, so zero calls already satisfies it...
        spy.assert();

        times_target(2);
        times_target(2);

        // ...and no amount of further calls can exceed it.
        spy.assert();
    }
}

mod mock {
    #[fnmock::mockable]
    fn times_target(id: i32) {
        let _ = id;
    }

    #[test]
    fn test_times_exact() {
        let mock = times_target_mock();
        mock.setup(|_id| ());
        mock.expect(fnmock::predicate::eq(2)).times(3);

        times_target(2);
        times_target(2);
        times_target(2);

        mock.assert();
    }

    #[test]
    #[should_panic(expected = "Too many calls of the spied function")]
    fn test_more_calls_than_times_allows_panics() {
        let mock = times_target_mock();
        mock.expect(fnmock::predicate::eq(2)).times(1);

        times_target(2);
        times_target(2);
    }
}
