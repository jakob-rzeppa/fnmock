mod fake {
    #[fnmock::fakeable]
    fn return_option(a: String) -> Option<String> {
        Some(a)
    }

    #[test]
    fn test_return_option() {
        let res = return_option("Test".to_string());
        assert_eq!(res, Some("Test".to_string()));
    }

    #[test]
    fn test_return_option_fake() {
        return_option_fake().setup(|a| Some(format!("Fake {}", a)));
        let res = return_option("Test".to_string());
        assert_eq!(res, Some("Fake Test".to_string()));
    }
}

mod spy {
    #[fnmock::spyable]
    fn return_option(a: String) -> Option<String> {
        Some(a)
    }

    #[test]
    fn test_return_option_spy() {
        let spy = return_option_spy();
        spy.expect(fnmock::predicate::eq("hi".to_string()));

        return_option("hi".to_string());

        spy.assert();
    }
}
