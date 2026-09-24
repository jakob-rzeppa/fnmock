//! A higher-ranked bound can be spelled two ways: with the binder inside the bound
//! (`F: for<'x> Fn(&'x str) -> String`, covered by `generics/higher_ranked_bounds.rs`) or
//! with the binder on the `where` predicate itself. The second spelling puts `'x` on the
//! predicate, so merging the predicate into the parameter has to carry the binder along —
//! otherwise `'x` is left dangling on the generated items.

mod fake {
    #[fnmock::fakeable]
    fn higher_ranked_where_predicate<F: 'static>(f: F, s: &str) -> String
    where
        for<'x> F: Fn(&'x str) -> String,
    {
        f(s)
    }

    fn uppercase(s: &str) -> String {
        s.to_uppercase()
    }

    #[test]
    fn test_higher_ranked_where_predicate() {
        let f: fn(&str) -> String = uppercase;
        let res = higher_ranked_where_predicate(f, "test");
        assert_eq!(res, "TEST");
    }

    #[test]
    fn test_higher_ranked_where_predicate_fake() {
        higher_ranked_where_predicate_fake::<fn(&str) -> String>()
            .setup(|_f, s| format!("Fake {s}"));

        let f: fn(&str) -> String = uppercase;
        let res = higher_ranked_where_predicate(f, "test");
        assert_eq!(res, "Fake test");
    }
}

mod spy {
    #[fnmock::spyable]
    fn higher_ranked_where_predicate<F: 'static>(f: F, s: &str) -> String
    where
        for<'x> F: Fn(&'x str) -> String,
    {
        f(s)
    }

    fn uppercase(s: &str) -> String {
        s.to_uppercase()
    }

    #[test]
    fn test_higher_ranked_where_predicate() {
        let spy = higher_ranked_where_predicate_spy::<fn(&str) -> String>();
        spy.expectf(|_f, s: &str| s == "test").once();

        let f: fn(&str) -> String = uppercase;
        let res = higher_ranked_where_predicate(f, "test");

        assert_eq!(res, "TEST");
        spy.assert();
    }
}

mod mock {
    #[fnmock::mockable]
    fn higher_ranked_where_predicate<F: 'static>(f: F, s: &str) -> String
    where
        for<'x> F: Fn(&'x str) -> String,
    {
        f(s)
    }

    fn uppercase(s: &str) -> String {
        s.to_uppercase()
    }

    #[test]
    fn test_higher_ranked_where_predicate() {
        let mock = higher_ranked_where_predicate_mock::<fn(&str) -> String>();
        mock.setup(|_f, s| format!("Fake {s}"));
        mock.expectf(|_f, s: &str| s == "test").once();

        let f: fn(&str) -> String = uppercase;
        let res = higher_ranked_where_predicate(f, "test");

        assert_eq!(res, "Fake test");
        mock.assert();
    }
}
