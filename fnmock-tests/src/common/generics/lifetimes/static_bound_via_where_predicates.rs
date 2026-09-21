mod fake {
    #[fnmock::fakeable]
    fn static_bound_via_where_predicates<'a, T>(value: T) -> String
    where
        T: 'a + std::fmt::Display,
        'a: 'static,
    {
        format!("{value}")
    }

    #[test]
    fn test_static_bound_via_where_predicates() {
        let res = static_bound_via_where_predicates("Test".to_string());
        assert_eq!(res, "Test");
    }

    #[test]
    fn test_static_bound_via_where_predicates_fake() {
        static_bound_via_where_predicates_fake::<String>().setup(|value| format!("Fake {value}"));

        let res = static_bound_via_where_predicates("Test".to_string());
        assert_eq!(res, "Fake Test");
    }
}

mod spy {
    #[fnmock::spyable]
    fn static_bound_via_where_predicates<'a, T>(value: T) -> String
    where
        T: 'a + std::fmt::Display,
        'a: 'static,
    {
        format!("{value}")
    }

    #[test]
    fn test_static_bound_via_where_predicates() {
        let spy = static_bound_via_where_predicates_spy::<String>();
        spy.expect(fnmock::predicate::eq("Test".to_string())).once();

        let res = static_bound_via_where_predicates("Test".to_string());

        assert_eq!(res, "Test");
        spy.assert();
    }
}
