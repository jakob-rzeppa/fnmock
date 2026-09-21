mod fake {
    struct Ref<'a>(&'a str);

    #[fnmock::fakeable]
    fn mixed_lifetime_and_generic<'a, T: 'static>(r: Ref<'a>, value: T) -> usize {
        let _ = value;
        r.0.len()
    }

    #[test]
    fn test_mixed_lifetime_and_generic() {
        let res = mixed_lifetime_and_generic(Ref("hi"), 2);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_mixed_lifetime_and_generic_fake() {
        mixed_lifetime_and_generic_fake::<i32>().setup(|r, value| {
            assert_eq!(r.0, "hi");
            assert_eq!(value, 2);
            3
        });

        let res = mixed_lifetime_and_generic(Ref("hi"), 2);
        assert_eq!(res, 3);
    }
}

mod spy {
    struct Ref<'a>(&'a str);

    #[fnmock::spyable]
    fn mixed_lifetime_and_generic<'a, T: 'static>(r: Ref<'a>, value: T) -> usize {
        let _ = value;
        r.0.len()
    }

    #[test]
    fn test_mixed_lifetime_and_generic() {
        let spy = mixed_lifetime_and_generic_spy::<i32>();
        spy.expectf(|r: &Ref<'_>, value: &i32| r.0 == "hi" && *value == 2)
            .once();

        let owned = "hi".to_string();
        let res = mixed_lifetime_and_generic(Ref(&owned), 2);

        assert_eq!(res, 2);
        spy.assert();
    }

    #[test]
    fn test_instantiations_stay_isolated_when_a_lifetime_is_present() {
        let spy_i32 = mixed_lifetime_and_generic_spy::<i32>();
        let spy_u8 = mixed_lifetime_and_generic_spy::<u8>();
        spy_i32.expect_once();
        spy_u8.expect_never();

        let owned = "hi".to_string();
        mixed_lifetime_and_generic(Ref(&owned), 2i32);

        spy_i32.assert();
        spy_u8.assert();
    }
}
