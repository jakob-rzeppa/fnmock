mod fake {
    #[fnmock::fakeable]
    fn reference(a: &str) -> String {
        a.to_string()
    }

    #[test]
    fn test_reference() {
        let value = "Test".to_string();
        let result = reference(&value);
        assert_eq!(result, "Test");
    }

    #[test]
    fn test_reference_fake() {
        let value = "Test".to_string();
        reference_fake().setup(|a| format!("{} fake modified", a));
        let result = reference(&value);
        assert_eq!(result, "Test fake modified");
    }
}

mod spy {
    #[fnmock::spyable]
    fn by_reference(value: &str) {
        let _ = value;
    }

    #[test]
    fn test_by_reference() {
        let spy = by_reference_spy();
        spy.expect(fnmock::predicate::eq("hi".to_string()));

        by_reference("hi");

        spy.assert();
    }
}
