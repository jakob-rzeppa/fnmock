mod fake {
    #[fnmock::fakeable]
    fn bound_referencing_a_static_lifetime<'a, T: Into<&'a str> + 'static>(value: T) -> usize
    where
        'a: 'static,
    {
        let s: &'a str = value.into();
        s.len()
    }

    #[test]
    fn test_bound_referencing_a_static_lifetime() {
        let res = bound_referencing_a_static_lifetime("Test");
        assert_eq!(res, 4);
    }

    #[test]
    fn test_bound_referencing_a_static_lifetime_fake() {
        bound_referencing_a_static_lifetime_fake::<&'static str>().setup(|_value| 42);

        let res = bound_referencing_a_static_lifetime("Test");
        assert_eq!(res, 42);
    }
}

mod spy {
    #[fnmock::spyable]
    fn bound_referencing_a_static_lifetime<'a, T: Into<&'a str> + 'static>(value: T) -> usize
    where
        'a: 'static,
    {
        let s: &'a str = value.into();
        s.len()
    }

    #[test]
    fn test_bound_referencing_a_static_lifetime() {
        let spy = bound_referencing_a_static_lifetime_spy::<&'static str>();
        spy.expectf(|value: &&str| *value == "Test").once();

        let res = bound_referencing_a_static_lifetime("Test");

        assert_eq!(res, 4);
        spy.assert();
    }
}

mod mock {
    #[fnmock::mockable]
    fn bound_referencing_a_static_lifetime<'a, T: Into<&'a str> + 'static>(value: T) -> usize
    where
        'a: 'static,
    {
        let s: &'a str = value.into();
        s.len()
    }

    #[test]
    fn test_bound_referencing_a_static_lifetime() {
        let mock = bound_referencing_a_static_lifetime_mock::<&'static str>();
        mock.setup(|_value| 42);
        mock.expectf(|value: &&str| *value == "Test").once();

        let res = bound_referencing_a_static_lifetime("Test");

        assert_eq!(res, 42);
        mock.assert();
    }
}
