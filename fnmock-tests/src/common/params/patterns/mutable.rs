mod fake {
    #[fnmock::fakeable]
    fn mutable(mut val: String) -> String {
        val.push_str(" prefix");
        val
    }

    #[test]
    fn test_mutable() {
        let result = mutable("Test".to_string());
        assert_eq!(result, "Test prefix");
    }

    #[test]
    fn test_mutable_fake() {
        mutable_fake().setup(|mut val| {
            val.push_str(" Fake prefix");
            val
        });

        let result = mutable("Test".to_string());
        assert_eq!(result, "Test Fake prefix");
    }
}

mod spy {
    #[fnmock::spyable]
    fn mutable(mut val: String) -> String {
        val.push_str(" prefix");
        val
    }

    #[test]
    fn test_mutable() {
        let spy = mutable_spy();
        spy.expect(fnmock::predicate::eq("hi".to_string()));
        spy.expectf(|val| val == "hi");

        let result = mutable("hi".to_string());

        assert_eq!(result, "hi prefix");
        spy.assert();
    }
}
