mod fake {
    #[fnmock::fakeable]
    fn static_bound_via_lifetime_param_declaration<'a: 'static, T: 'a + std::fmt::Display>(
        value: T,
    ) -> String {
        format!("{value}")
    }

    #[test]
    fn test_static_bound_via_lifetime_param_declaration() {
        let res = static_bound_via_lifetime_param_declaration("Test".to_string());
        assert_eq!(res, "Test");
    }

    #[test]
    fn test_static_bound_via_lifetime_param_declaration_fake() {
        static_bound_via_lifetime_param_declaration_fake::<String>()
            .setup(|value| format!("Fake {value}"));

        let res = static_bound_via_lifetime_param_declaration("Test".to_string());
        assert_eq!(res, "Fake Test");
    }
}

mod spy {
    #[fnmock::spyable]
    fn static_bound_via_lifetime_param_declaration<'a: 'static, T: 'a + std::fmt::Display>(
        value: T,
    ) -> String {
        format!("{value}")
    }

    #[test]
    fn test_static_bound_via_lifetime_param_declaration() {
        let spy = static_bound_via_lifetime_param_declaration_spy::<String>();
        spy.expect(fnmock::predicate::eq("Test".to_string())).once();

        let res = static_bound_via_lifetime_param_declaration("Test".to_string());

        assert_eq!(res, "Test");
        spy.assert();
    }
}

mod mock {
    #[fnmock::mockable]
    fn static_bound_via_lifetime_param_declaration<'a: 'static, T: 'a + std::fmt::Display>(
        value: T,
    ) -> String {
        format!("{value}")
    }

    #[test]
    fn test_static_bound_via_lifetime_param_declaration() {
        let mock = static_bound_via_lifetime_param_declaration_mock::<String>();
        mock.setup(|value| format!("Fake {value}"));
        mock.expect(fnmock::predicate::eq("Test".to_string()))
            .once();

        let res = static_bound_via_lifetime_param_declaration("Test".to_string());

        assert_eq!(res, "Fake Test");
        mock.assert();
    }
}
