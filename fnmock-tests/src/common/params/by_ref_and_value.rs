mod fake {
    #[fnmock::fakeable]
    fn by_ref_and_value(mut a: String, b: &str) -> String {
        a.push_str(b);
        a
    }

    #[test]
    fn test_by_ref_and_value() {
        let res = by_ref_and_value("Test".to_string(), " Case");
        assert_eq!(res, "Test Case");
    }

    #[test]
    fn test_by_ref_and_value_fake() {
        by_ref_and_value_fake().setup(|a, b| format!("Fake {}{}", a, b));
        let res = by_ref_and_value("Test".to_string(), " Case");
        assert_eq!(res, "Fake Test Case");
    }
}

mod spy {
    #[fnmock::spyable]
    fn by_ref_and_value(mut a: String, b: &str) -> String {
        a.push_str(b);
        a
    }

    #[test]
    fn test_by_ref_and_value() {
        let spy = by_ref_and_value_spy();
        spy.expect(
            fnmock::predicate::eq("hi".to_string()),
            fnmock::predicate::eq(" there"),
        );

        by_ref_and_value("hi".to_string(), " there");

        spy.assert();
    }
}

mod mock {
    #[fnmock::mockable]
    fn by_ref_and_value(mut a: String, b: &str) -> String {
        a.push_str(b);
        a
    }

    #[test]
    fn test_by_ref_and_value() {
        let mock = by_ref_and_value_mock();
        mock.setup(|a, b| format!("Fake {}{}", a, b));
        mock.expect(
            fnmock::predicate::eq("hi".to_string()),
            fnmock::predicate::eq(" there"),
        )
        .once();

        let res = by_ref_and_value("hi".to_string(), " there");

        assert_eq!(res, "Fake hi there");
        mock.assert();
    }
}
