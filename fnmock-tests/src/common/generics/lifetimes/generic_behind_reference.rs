mod fake {
    #[fnmock::fakeable]
    fn generic_behind_reference<'a, T: 'static + std::fmt::Debug>(value: &'a T) -> String {
        format!("{value:?}")
    }

    #[test]
    fn test_generic_behind_reference() {
        let owned = 1u8;
        let res = generic_behind_reference(&owned);
        assert_eq!(res, "1");
    }

    #[test]
    fn test_generic_behind_reference_fake() {
        generic_behind_reference_fake::<u8>().setup(|_value| "Fake".to_string());

        let owned = 1u8;
        let res = generic_behind_reference(&owned);
        assert_eq!(res, "Fake");
    }
}

mod spy {
    #[fnmock::spyable]
    fn generic_behind_reference<'a, T: 'static + std::fmt::Debug>(value: &'a T) -> String {
        format!("{value:?}")
    }

    #[test]
    fn test_generic_behind_reference() {
        let spy = generic_behind_reference_spy::<u8>();
        spy.expect(fnmock::predicate::eq(1u8)).once();

        let owned = 1u8;
        let res = generic_behind_reference(&owned);

        assert_eq!(res, "1");
        spy.assert();
    }
}
