mod fake {
    #[fnmock::fakeable]
    fn where_bounds<T, U>(a: T, b: U) -> String
    where
        T: std::fmt::Display + 'static,
        U: std::fmt::Debug + 'static,
    {
        format!("{} {:?}", a, b)
    }

    #[test]
    fn test_where_bounds() {
        let res = where_bounds("Test".to_string(), 2);
        assert_eq!(res, "Test 2");
    }

    #[test]
    fn test_where_bounds_fake() {
        where_bounds_fake::<String, i32>().setup(|a, b| format!("Fake {} {:?}", a, b));
        let res = where_bounds("Test".to_string(), 2);
        assert_eq!(res, "Fake Test 2");
    }
}

mod spy {
    #[fnmock::spyable]
    fn where_bounds<T, U>(a: T, b: U) -> String
    where
        T: std::fmt::Display + 'static,
        U: std::fmt::Debug + 'static,
    {
        format!("{} {:?}", a, b)
    }

    #[test]
    fn test_where_bounds() {
        let spy = where_bounds_spy::<String, i32>();
        spy.expect(
            fnmock::predicate::eq("hi".to_string()),
            fnmock::predicate::eq(2),
        )
        .once();

        let res = where_bounds("hi".to_string(), 2);

        assert_eq!(res, "hi 2");
        spy.assert();
    }
}

mod mock {
    #[fnmock::mockable]
    fn where_bounds<T, U>(a: T, b: U) -> String
    where
        T: std::fmt::Display + 'static,
        U: std::fmt::Debug + 'static,
    {
        format!("{} {:?}", a, b)
    }

    #[test]
    fn test_where_bounds() {
        let mock = where_bounds_mock::<String, i32>();
        mock.setup(|a, b| format!("Fake {} {:?}", a, b));
        mock.expect(
            fnmock::predicate::eq("hi".to_string()),
            fnmock::predicate::eq(2),
        )
        .once();

        let res = where_bounds("hi".to_string(), 2);

        assert_eq!(res, "Fake hi 2");
        mock.assert();
    }
}
