mod fake {
    #[fnmock::fakeable]
    fn return_unit(a: &mut String) {
        a.push_str(" modified");
    }

    #[fnmock::fakeable]
    fn return_unit_explicitly(a: &mut String) -> () {
        a.push_str(" modified");
    }

    #[test]
    fn test_return_unit() {
        let mut value = "Test".to_string();
        return_unit(&mut value);
        assert_eq!(value, "Test modified");
    }

    #[test]
    fn test_return_unit_fake() {
        let mut value = "Test".to_string();
        return_unit_fake().setup(|a| a.push_str(" fake modified"));
        return_unit(&mut value);
        assert_eq!(value, "Test fake modified");
    }

    #[test]
    fn test_return_unit_explicitly() {
        let mut value = "Test".to_string();
        return_unit_explicitly(&mut value);
        assert_eq!(value, "Test modified");
    }

    #[test]
    fn test_return_unit_explicitly_fake() {
        let mut value = "Test".to_string();
        return_unit_explicitly_fake().setup(|a| a.push_str(" fake modified"));
        return_unit_explicitly(&mut value);
        assert_eq!(value, "Test fake modified");
    }
}

mod spy {
    #[fnmock::spyable]
    fn return_unit(a: &mut String) {
        a.push_str(" modified");
    }

    #[test]
    fn test_return_unit_spy() {
        let spy = return_unit_spy();
        spy.expect(fnmock::predicate::eq("Test".to_string()));

        let mut value = "Test".to_string();
        return_unit(&mut value);

        spy.assert();
        assert_eq!(value, "Test modified");
    }
}
