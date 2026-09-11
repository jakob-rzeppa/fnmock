mod fake {
    #[fnmock::fakeable]
    fn where_bounds<T: 'static, U: std::fmt::Debug>(a: T, b: U) -> String
    where
        T: std::fmt::Display,
        U: 'static,
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
    fn where_bounds<T: 'static, U: std::fmt::Debug>(a: T, b: U) -> String
    where
        T: std::fmt::Display,
        U: 'static,
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
