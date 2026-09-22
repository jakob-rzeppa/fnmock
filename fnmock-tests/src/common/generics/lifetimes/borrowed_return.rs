mod fake {
    #[fnmock::fakeable]
    fn borrowed_return<'a>(s: &'a str) -> &'a str {
        s
    }

    #[test]
    fn test_borrowed_return() {
        let owned = "Test".to_string();
        let res = borrowed_return(&owned);
        assert_eq!(res, "Test");
    }

    #[test]
    fn test_borrowed_return_fake() {
        borrowed_return_fake().setup(|_s| "Fake");

        let owned = "Test".to_string();
        let res = borrowed_return(&owned);
        assert_eq!(res, "Fake");
    }

    #[test]
    fn test_borrowed_return_fake_can_return_its_argument() {
        borrowed_return_fake().setup(|s| s);

        let owned = "Test".to_string();
        let res = borrowed_return(&owned);
        assert_eq!(res, "Test");
    }
}

mod spy {
    #[fnmock::spyable]
    fn borrowed_return<'a>(s: &'a str) -> &'a str {
        s
    }

    #[test]
    fn test_borrowed_return() {
        let spy = borrowed_return_spy();
        spy.expect(fnmock::predicate::eq("Test".to_string())).once();

        let owned = "Test".to_string();
        let res = borrowed_return(&owned);

        assert_eq!(res, "Test");
        spy.assert();
    }
}

mod mock {
    #[fnmock::mockable]
    fn borrowed_return<'a>(s: &'a str) -> &'a str {
        s
    }

    #[test]
    fn test_borrowed_return() {
        let mock = borrowed_return_mock();
        mock.setup(|_s| "Fake");
        mock.expect(fnmock::predicate::eq("Test".to_string()))
            .once();

        let owned = "Test".to_string();
        let res = borrowed_return(&owned);

        assert_eq!(res, "Fake");
        mock.assert();
    }
}
