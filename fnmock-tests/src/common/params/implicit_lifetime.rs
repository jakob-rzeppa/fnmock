mod fake {
    #[fnmock::fakeable]
    fn implicit_lifetime(a: &'_ str) -> String {
        a.to_string()
    }

    #[test]
    fn test_implicit_lifetime() {
        let value = "Test".to_string();
        let result = implicit_lifetime(&value);
        assert_eq!(result, "Test");
    }

    #[test]
    fn test_implicit_lifetime_fake() {
        let value = "Test".to_string();
        implicit_lifetime_fake().setup(|a| format!("{} fake modified", a));
        let result = implicit_lifetime(&value);
        assert_eq!(result, "Test fake modified");
    }
}

mod spy {
    #[fnmock::spyable]
    fn implicit_lifetime(a: &'_ str) -> String {
        a.to_string()
    }

    #[test]
    fn test_implicit_lifetime_spy() {
        let spy = implicit_lifetime_spy();
        spy.expect(fnmock::predicate::eq("Test".to_string()));

        let value = "Test".to_string();
        let result = implicit_lifetime(&value);
        assert_eq!(result, "Test");
        spy.assert();
    }
}
