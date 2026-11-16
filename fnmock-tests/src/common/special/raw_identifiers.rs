//! Raw identifiers (`r#type`) as function, method and parameter names. The generated names drop the
//! `r#` prefix, e.g. `fn r#type` gets the accessor `type_fake`.

mod fake {
    #[fnmock::fakeable]
    fn r#type(r#match: u32) -> u32 {
        r#match + 1
    }

    struct Keywords;

    #[fnmock::fakeable]
    impl Keywords {
        fn r#loop(&self, r#in: u32) -> u32 {
            r#in + 1
        }
    }

    #[test]
    fn test_raw_identifier_function() {
        assert_eq!(r#type(1), 2);
    }

    #[test]
    fn test_raw_identifier_function_fake() {
        type_fake().setup(|r#match| r#match * 10);

        assert_eq!(r#type(1), 10);
    }

    #[test]
    fn test_raw_identifier_method_fake() {
        Keywords::loop_fake().setup(|_, r#in| r#in * 10);

        assert_eq!(Keywords.r#loop(1), 10);
    }
}

mod spy {
    #[fnmock::spyable]
    fn r#type(r#match: u32) -> u32 {
        r#match + 1
    }

    struct Keywords;

    #[fnmock::spyable]
    impl Keywords {
        fn r#loop(&self, r#in: u32) -> u32 {
            r#in + 1
        }
    }

    #[test]
    fn test_raw_identifier_function_spy() {
        let spy = type_spy();
        spy.expect(fnmock::predicate::eq(1)).once();

        assert_eq!(r#type(1), 2);
        spy.assert();
    }

    #[test]
    #[should_panic(expected = "match == 1")]
    fn test_matcher_display_drops_raw_prefix() {
        let spy = type_spy();
        spy.expect(fnmock::predicate::eq(1)).once();

        spy.assert();
    }

    #[test]
    fn test_raw_identifier_method_spy() {
        let spy = Keywords::loop_spy();
        spy.expect(fnmock::predicate::eq(1)).once();

        assert_eq!(Keywords.r#loop(1), 2);
        spy.assert();
    }
}

mod mock {
    #[fnmock::mockable]
    fn r#type(r#match: u32) -> u32 {
        r#match + 1
    }

    struct Keywords;

    #[fnmock::mockable]
    impl Keywords {
        fn r#loop(&self, r#in: u32) -> u32 {
            r#in + 1
        }
    }

    #[test]
    fn test_raw_identifier_function_mock() {
        let mock = type_mock();
        mock.setup(|r#match| r#match * 10);
        mock.expect(fnmock::predicate::eq(1)).once();

        assert_eq!(r#type(1), 10);
        mock.assert();
    }

    #[test]
    fn test_raw_identifier_method_mock() {
        let mock = Keywords::loop_mock();
        mock.setup(|_, r#in| r#in * 10);
        mock.expect(fnmock::predicate::eq(1)).once();

        assert_eq!(Keywords.r#loop(1), 10);
        mock.assert();
    }
}
