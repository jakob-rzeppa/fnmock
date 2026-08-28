mod fake {
    #[fnmock::fakeable]
    fn reference_with_named_lifetime<'a>(s: &'a str) -> usize {
        s.len()
    }

    #[test]
    fn test_reference_with_named_lifetime() {
        let res = reference_with_named_lifetime("Test");
        assert_eq!(res, 4);
    }

    #[test]
    fn test_reference_with_named_lifetime_fake() {
        reference_with_named_lifetime_fake().setup(|s| s.len() + 1);
        let res = reference_with_named_lifetime("Test");
        assert_eq!(res, 5);
    }
}

mod spy {
    #[fnmock::spyable]
    fn reference_with_named_lifetime<'a>(s: &'a str) -> usize {
        s.len()
    }

    #[test]
    fn test_expect_still_works_for_a_plain_reference() {
        let spy = reference_with_named_lifetime_spy();
        spy.expect(fnmock::predicate::eq("hi".to_string())).once();

        let res = reference_with_named_lifetime("hi");

        assert_eq!(res, 2);
        spy.assert();
    }

    #[test]
    fn test_expectf_still_works_for_a_plain_reference() {
        let spy = reference_with_named_lifetime_spy();
        spy.expectf(|s: &str| s.starts_with('h')).once();

        reference_with_named_lifetime("hi");

        spy.assert();
    }
}
