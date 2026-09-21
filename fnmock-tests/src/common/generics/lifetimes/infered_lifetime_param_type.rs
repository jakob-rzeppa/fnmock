mod fake {
    struct Ref<'a>(&'a str);

    #[fnmock::fakeable]
    fn elided_lifetime_param_type(r: Ref<'_>) -> usize {
        r.0.len()
    }

    #[test]
    fn test_elided_lifetime_param_type() {
        let res = elided_lifetime_param_type(Ref("hi"));
        assert_eq!(res, 2);
    }

    #[test]
    fn test_elided_lifetime_param_type_fake() {
        elided_lifetime_param_type_fake().setup(|r: Ref<'_>| {
            assert_eq!(r.0, "hi");
            42
        });

        let res = elided_lifetime_param_type(Ref("hi"));
        assert_eq!(res, 42);
    }
}

mod spy {
    struct Ref<'a>(&'a str);

    #[fnmock::spyable]
    fn elided_lifetime_param_type(r: Ref<'_>) -> usize {
        r.0.len()
    }

    #[test]
    fn test_elided_lifetime_param_type() {
        let spy = elided_lifetime_param_type_spy();
        spy.expectf(|r: &Ref<'_>| r.0 == "hi").once();

        let owned = "hi".to_string();
        let res = elided_lifetime_param_type(Ref(&owned));

        assert_eq!(res, 2);
        spy.assert();
    }
}
