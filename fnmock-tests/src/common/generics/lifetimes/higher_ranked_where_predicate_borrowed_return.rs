//! The binder on a `where` predicate can tie the argument and the return together
//! (`for<'x> F: Fn(&'x str) -> &'x str`). Carrying the binder along has to keep both uses of
//! `'x` bound by the same `for<..>`, not just the one in argument position.

mod fake {
    #[fnmock::fakeable]
    fn higher_ranked_where_predicate_borrowed_return<F: 'static>(f: F, s: &str) -> usize
    where
        for<'x> F: Fn(&'x str) -> &'x str,
    {
        f(s).len()
    }

    fn identity(s: &str) -> &str {
        s
    }

    #[test]
    fn test_higher_ranked_where_predicate_borrowed_return() {
        let f: fn(&str) -> &str = identity;
        let res = higher_ranked_where_predicate_borrowed_return(f, "test");
        assert_eq!(res, 4);
    }

    #[test]
    fn test_higher_ranked_where_predicate_borrowed_return_fake() {
        higher_ranked_where_predicate_borrowed_return_fake::<fn(&str) -> &str>()
            .setup(|_f, s| s.len() + 1);

        let f: fn(&str) -> &str = identity;
        let res = higher_ranked_where_predicate_borrowed_return(f, "test");
        assert_eq!(res, 5);
    }
}

mod spy {
    #[fnmock::spyable]
    fn higher_ranked_where_predicate_borrowed_return<F: 'static>(f: F, s: &str) -> usize
    where
        for<'x> F: Fn(&'x str) -> &'x str,
    {
        f(s).len()
    }

    fn identity(s: &str) -> &str {
        s
    }

    #[test]
    fn test_higher_ranked_where_predicate_borrowed_return() {
        let spy = higher_ranked_where_predicate_borrowed_return_spy::<fn(&str) -> &str>();
        spy.expectf(|_f, s: &str| s == "test").once();

        let f: fn(&str) -> &str = identity;
        let res = higher_ranked_where_predicate_borrowed_return(f, "test");

        assert_eq!(res, 4);
        spy.assert();
    }
}
