mod fake {
    #[fnmock::fakeable]
    fn reference_in_option(a: Option<&str>) -> String {
        format!("Real {}", a.unwrap_or("none"))
    }

    #[test]
    fn test_reference_in_option() {
        let value = "Test".to_string();
        assert_eq!(reference_in_option(Some(&value)), "Real Test");
        assert_eq!(reference_in_option(None), "Real none");
    }

    #[test]
    fn test_reference_in_option_fake() {
        reference_in_option_fake().setup(|a| format!("Fake {}", a.unwrap_or("none")));

        let value = "Test".to_string();
        assert_eq!(reference_in_option(Some(&value)), "Fake Test");
        assert_eq!(reference_in_option(None), "Fake none");
    }
}

mod spy {
    #[fnmock::spyable]
    fn reference_in_option(a: Option<&'static str>) -> String {
        format!("Real {}", a.unwrap_or("none"))
    }

    #[test]
    fn test_reference_in_option_spy() {
        let spy = reference_in_option_spy();
        spy.expect(fnmock::predicate::eq(Some("hi")));

        reference_in_option(Some("hi"));

        spy.assert();
    }
}
