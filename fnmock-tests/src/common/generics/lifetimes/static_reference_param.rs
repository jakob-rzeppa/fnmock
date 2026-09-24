mod fake {
    #[fnmock::fakeable]
    fn static_reference_param(s: &'static str) -> usize {
        s.len()
    }

    #[test]
    fn test_static_reference_param() {
        let res = static_reference_param("Test");
        assert_eq!(res, 4);
    }

    #[test]
    fn test_static_reference_param_fake() {
        static_reference_param_fake().setup(|s| s.len() + 1);

        let res = static_reference_param("Test");
        assert_eq!(res, 5);
    }
}

mod spy {
    #[fnmock::spyable]
    fn static_reference_param(s: &'static str) -> usize {
        s.len()
    }

    #[test]
    fn test_static_reference_param() {
        let spy = static_reference_param_spy();
        spy.expect(fnmock::predicate::eq("Test".to_string())).once();

        let res = static_reference_param("Test");

        assert_eq!(res, 4);
        spy.assert();
    }
}

mod mock {
    #[fnmock::mockable]
    fn static_reference_param(s: &'static str) -> usize {
        s.len()
    }

    #[test]
    fn test_static_reference_param() {
        let mock = static_reference_param_mock();
        mock.setup(|s| s.len() + 1);
        mock.expect(fnmock::predicate::eq("Test".to_string()))
            .once();

        let res = static_reference_param("Test");

        assert_eq!(res, 5);
        mock.assert();
    }
}
