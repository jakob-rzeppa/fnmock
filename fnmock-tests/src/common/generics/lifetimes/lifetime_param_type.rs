mod fake {
    struct Ref<'a>(&'a str);

    #[fnmock::fakeable]
    fn lifetime_param_type<'a>(r: Ref<'a>) -> usize {
        r.0.len()
    }

    #[test]
    fn test_lifetime_param_type() {
        let owned = "hi".to_string();
        let res = lifetime_param_type(Ref(&owned));
        assert_eq!(res, 2);
    }

    #[test]
    fn test_lifetime_param_type_fake() {
        lifetime_param_type_fake().setup(|r: Ref<'_>| r.0.len() + 1);
        let owned = "hi".to_string();
        let res = lifetime_param_type(Ref(&owned));
        assert_eq!(res, 3);
    }
}

mod spy {
    struct Ref<'a>(&'a str);

    #[fnmock::spyable]
    fn lifetime_param_type<'a>(r: Ref<'a>) -> usize {
        r.0.len()
    }

    #[test]
    fn test_expectf_matches_a_lifetime_parameterized_param() {
        let spy = lifetime_param_type_spy();
        spy.expectf(|r: &Ref<'_>| r.0 == "hi").once();

        let owned = "hi".to_string();
        let res = lifetime_param_type(Ref(&owned));

        assert_eq!(res, 2);
        spy.assert();
    }

    #[test]
    fn test_expectf_sees_a_shorter_borrow_than_the_expectation() {
        let spy = lifetime_param_type_spy();
        spy.expectf(|r: &Ref<'_>| !r.0.is_empty()).times(2);

        {
            let short = "a".to_string();
            lifetime_param_type(Ref(&short));
        }
        {
            let shorter = "b".to_string();
            lifetime_param_type(Ref(&shorter));
        }

        spy.assert();
    }

    #[test]
    #[should_panic(expected = "Expectation(s) of the spied function")]
    fn test_unmatched_expectf_fails_assert() {
        let spy = lifetime_param_type_spy();
        spy.expectf(|r: &Ref<'_>| r.0 == "nope").once();

        let owned = "hi".to_string();
        lifetime_param_type(Ref(&owned));

        spy.assert();
    }
}
