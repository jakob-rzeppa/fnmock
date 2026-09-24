mod fake {
    #[fnmock::fakeable]
    fn static_bound_via_transitive_lifetime<'a: 'static, 'b: 'a, T: 'b + std::fmt::Display>(
        value: T,
    ) -> String {
        format!("{value}")
    }

    #[test]
    fn test_static_bound_via_transitive_lifetime() {
        let res = static_bound_via_transitive_lifetime("Test".to_string());
        assert_eq!(res, "Test");
    }

    #[test]
    fn test_static_bound_via_transitive_lifetime_fake() {
        static_bound_via_transitive_lifetime_fake::<String>()
            .setup(|value| format!("Fake {value}"));

        let res = static_bound_via_transitive_lifetime("Test".to_string());
        assert_eq!(res, "Fake Test");
    }
}

mod spy {
    #[fnmock::spyable]
    fn static_bound_via_transitive_lifetime<'a: 'static, 'b: 'a, T: 'b + std::fmt::Display>(
        value: T,
    ) -> String {
        format!("{value}")
    }

    #[test]
    fn test_static_bound_via_transitive_lifetime() {
        let spy = static_bound_via_transitive_lifetime_spy::<String>();
        spy.expect(fnmock::predicate::eq("Test".to_string())).once();

        let res = static_bound_via_transitive_lifetime("Test".to_string());

        assert_eq!(res, "Test");
        spy.assert();
    }
}

mod mock {
    #[fnmock::mockable]
    fn static_bound_via_transitive_lifetime<'a: 'static, 'b: 'a, T: 'b + std::fmt::Display>(
        value: T,
    ) -> String {
        format!("{value}")
    }

    #[test]
    fn test_static_bound_via_transitive_lifetime() {
        let mock = static_bound_via_transitive_lifetime_mock::<String>();
        mock.setup(|value| format!("Fake {value}"));
        mock.expect(fnmock::predicate::eq("Test".to_string()))
            .once();

        let res = static_bound_via_transitive_lifetime("Test".to_string());

        assert_eq!(res, "Fake Test");
        mock.assert();
    }
}
