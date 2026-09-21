mod fake {
    struct HigherRankedWherePredicate;

    #[fnmock::fakeable]
    impl HigherRankedWherePredicate {
        fn apply<F: 'static>(&self, f: F, s: &str) -> String
        where
            for<'x> F: Fn(&'x str) -> String,
        {
            f(s)
        }
    }

    fn uppercase(s: &str) -> String {
        s.to_uppercase()
    }

    #[test]
    fn test_higher_ranked_where_predicate() {
        let f: fn(&str) -> String = uppercase;
        assert_eq!(HigherRankedWherePredicate.apply(f, "test"), "TEST");
    }

    #[test]
    fn test_higher_ranked_where_predicate_fake() {
        HigherRankedWherePredicate::apply_fake::<fn(&str) -> String>()
            .setup(|_, _f, s| format!("Fake {s}"));

        let f: fn(&str) -> String = uppercase;
        assert_eq!(HigherRankedWherePredicate.apply(f, "test"), "Fake test");
    }
}

mod spy {
    struct HigherRankedWherePredicate;

    #[fnmock::spyable]
    impl HigherRankedWherePredicate {
        fn apply<F: 'static>(&self, f: F, s: &str) -> String
        where
            for<'x> F: Fn(&'x str) -> String,
        {
            f(s)
        }
    }

    fn uppercase(s: &str) -> String {
        s.to_uppercase()
    }

    #[test]
    fn test_higher_ranked_where_predicate_spy() {
        let spy = HigherRankedWherePredicate::apply_spy::<fn(&str) -> String>();
        spy.expect_once();

        let f: fn(&str) -> String = uppercase;
        assert_eq!(HigherRankedWherePredicate.apply(f, "test"), "TEST");

        spy.assert();
    }
}
