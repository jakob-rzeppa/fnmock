mod fake {
    #[fnmock::fakeable]
    fn const_generic_with_lifetime<'a, const N: usize>(s: &'a str) -> usize {
        s.len() + N
    }

    #[test]
    fn test_const_generic_with_lifetime() {
        let res = const_generic_with_lifetime::<2>("Test");
        assert_eq!(res, 6);
    }

    #[test]
    fn test_const_generic_with_lifetime_fake() {
        const_generic_with_lifetime_fake::<2>().setup(|s| s.len() + 10);

        let res = const_generic_with_lifetime::<2>("Test");
        assert_eq!(res, 14);
    }

    #[test]
    fn test_const_values_stay_isolated_when_a_lifetime_is_present() {
        const_generic_with_lifetime_fake::<2>().setup(|s| s.len() + 10);

        assert_eq!(const_generic_with_lifetime::<2>("Test"), 14);
        assert_eq!(const_generic_with_lifetime::<3>("Test"), 7);
    }
}

mod spy {
    #[fnmock::spyable]
    fn const_generic_with_lifetime<'a, const N: usize>(s: &'a str) -> usize {
        s.len() + N
    }

    #[test]
    fn test_const_generic_with_lifetime() {
        let spy = const_generic_with_lifetime_spy::<2>();
        spy.expect(fnmock::predicate::eq("Test".to_string())).once();

        let res = const_generic_with_lifetime::<2>("Test");

        assert_eq!(res, 6);
        spy.assert();
    }
}
