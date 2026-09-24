//! The predicate surface fnmock re-exports has to work as an argument matcher, not just the
//! `eq`/`always` the rest of the suite leans on. `expect` takes any
//! [`Predicate<T>`](fnmock::Predicate), so the combinators from `fnmock::PredicateBooleanExt`
//! and the constructors under `fnmock::predicate` are part of fnmock's public API — including
//! their `Display`, which is what an unnamed expectation is reported by when it goes
//! unfulfilled.

mod spy {
    #[fnmock::spyable]
    fn predicate_num(id: i32) {
        let _ = id;
    }

    #[fnmock::spyable]
    fn predicate_str(name: &str) {
        let _ = name;
    }

    #[test]
    fn test_ordering_predicates() {
        let spy = predicate_num_spy();
        spy.expect(fnmock::predicate::gt(10)).once();
        spy.expect(fnmock::predicate::lt(0)).once();

        predicate_num(11);
        predicate_num(-1);
        predicate_num(5); // matches neither

        spy.assert();
    }

    #[test]
    fn test_ne_predicate() {
        let spy = predicate_num_spy();
        spy.expect(fnmock::predicate::ne(1)).times(2);

        predicate_num(0);
        predicate_num(1);
        predicate_num(2);

        spy.assert();
    }

    #[test]
    fn test_in_iter_predicate() {
        let spy = predicate_num_spy();
        spy.expect(fnmock::predicate::in_iter(vec![1, 2, 3]))
            .times(2);

        predicate_num(1);
        predicate_num(3);
        predicate_num(9);

        spy.assert();
    }

    #[test]
    fn test_function_predicate() {
        let spy = predicate_num_spy();
        spy.expect(fnmock::predicate::function(|n: &i32| n % 2 == 0))
            .times(2);

        predicate_num(2);
        predicate_num(4);
        predicate_num(5);

        spy.assert();
    }

    #[test]
    fn test_never_predicate_matches_nothing() {
        let spy = predicate_num_spy();
        spy.expect(fnmock::predicate::never()).never();

        predicate_num(1);

        spy.assert();
    }

    #[test]
    fn test_boolean_and_narrows_to_a_range() {
        use fnmock::PredicateBooleanExt;

        let spy = predicate_num_spy();
        spy.expect(fnmock::predicate::gt(0).and(fnmock::predicate::lt(10)))
            .once();

        predicate_num(5);
        predicate_num(50); // fails the right-hand side
        predicate_num(-5); // fails the left-hand side

        spy.assert();
    }

    #[test]
    fn test_boolean_or_widens_to_two_values() {
        use fnmock::PredicateBooleanExt;

        let spy = predicate_num_spy();
        spy.expect(fnmock::predicate::eq(1).or(fnmock::predicate::eq(2)))
            .times(2);

        predicate_num(1);
        predicate_num(2);
        predicate_num(3);

        spy.assert();
    }

    #[test]
    fn test_boolean_not_inverts() {
        use fnmock::PredicateBooleanExt;

        let spy = predicate_num_spy();
        spy.expect(fnmock::predicate::eq(1).not()).times(2);

        predicate_num(0);
        predicate_num(1);
        predicate_num(2);

        spy.assert();
    }

    #[test]
    fn test_str_predicates_on_a_str_parameter() {
        let spy = predicate_str_spy();
        spy.expect(fnmock::predicate::str::starts_with("he")).once();
        spy.expect(fnmock::predicate::str::contains("or")).once();

        predicate_str("hello");
        predicate_str("world");

        spy.assert();
    }

    #[test]
    #[should_panic(expected = "Expectation(s) of the spied function")]
    fn test_an_unmatched_combinator_predicate_fails_assert() {
        use fnmock::PredicateBooleanExt;

        let spy = predicate_num_spy();
        spy.expect(fnmock::predicate::gt(0).and(fnmock::predicate::lt(10)))
            .once();

        predicate_num(50);

        spy.assert();
    }
}

mod mock {
    #[fnmock::mockable]
    fn predicate_num(id: i32) {
        let _ = id;
    }

    #[test]
    fn test_ordering_predicates() {
        let mock = predicate_num_mock();
        mock.expect(fnmock::predicate::gt(10)).once();
        mock.expect(fnmock::predicate::lt(0)).once();

        predicate_num(11);
        predicate_num(-1);
        predicate_num(5); // matches neither

        mock.assert();
    }

    #[test]
    fn test_boolean_and_narrows_to_a_range() {
        use fnmock::PredicateBooleanExt;

        let mock = predicate_num_mock();
        mock.expect(fnmock::predicate::gt(0).and(fnmock::predicate::lt(10)))
            .once();

        predicate_num(5);
        predicate_num(50); // fails the right-hand side
        predicate_num(-5); // fails the left-hand side

        mock.assert();
    }
}
