mod fake {
    struct Ref<'a>(&'a str);

    struct LifetimesParamType<'s> {
        prefix: &'s str,
    }

    #[fnmock::fakeable]
    impl<'s> LifetimesParamType<'s> {
        fn len_of<'a>(&self, r: Ref<'a>) -> usize {
            self.prefix.len() + r.0.len()
        }

        fn len_with(&self, other: Ref<'s>) -> usize {
            self.prefix.len() + other.0.len()
        }

        fn len_elided(&self, r: Ref<'_>) -> usize {
            self.prefix.len() + r.0.len()
        }
    }

    #[test]
    fn test_lifetimes_param_type() {
        let s = LifetimesParamType { prefix: "ab" };
        assert_eq!(s.len_of(Ref("cde")), 5);
        assert_eq!(s.len_with(Ref("cde")), 5);
        assert_eq!(s.len_elided(Ref("cde")), 5);
    }

    #[test]
    fn test_lifetimes_param_type_fake() {
        LifetimesParamType::len_of_fake().setup(|_, r: Ref<'_>| r.0.len());
        LifetimesParamType::len_with_fake().setup(|_, r: Ref<'_>| r.0.len());
        LifetimesParamType::len_elided_fake().setup(|_, r: Ref<'_>| r.0.len());

        let s = LifetimesParamType { prefix: "ab" };
        assert_eq!(s.len_of(Ref("cde")), 3);
        assert_eq!(s.len_with(Ref("cde")), 3);
        assert_eq!(s.len_elided(Ref("cde")), 3);
    }
}

mod spy {
    struct Ref<'a>(&'a str);

    struct LifetimesParamType<'s> {
        prefix: &'s str,
    }

    #[fnmock::spyable]
    impl<'s> LifetimesParamType<'s> {
        fn len_of<'a>(&self, r: Ref<'a>) -> usize {
            self.prefix.len() + r.0.len()
        }

        fn len_with(&self, other: Ref<'s>) -> usize {
            self.prefix.len() + other.0.len()
        }

        fn len_elided(&self, r: Ref<'_>) -> usize {
            self.prefix.len() + r.0.len()
        }
    }

    #[test]
    fn test_lifetimes_param_type_spy() {
        let of_spy = LifetimesParamType::len_of_spy();
        let with_spy = LifetimesParamType::len_with_spy();
        let elided_spy = LifetimesParamType::len_elided_spy();
        of_spy.expect_once();
        with_spy.expect_once();
        elided_spy.expect_once();

        let s = LifetimesParamType { prefix: "ab" };
        assert_eq!(s.len_of(Ref("cde")), 5);
        assert_eq!(s.len_with(Ref("cde")), 5);
        assert_eq!(s.len_elided(Ref("cde")), 5);

        of_spy.assert();
        with_spy.assert();
        elided_spy.assert();
    }
}

mod mock {
    struct Ref<'a>(&'a str);

    struct LifetimesParamType<'s> {
        prefix: &'s str,
    }

    #[fnmock::mockable]
    impl<'s> LifetimesParamType<'s> {
        fn len_of<'a>(&self, r: Ref<'a>) -> usize {
            self.prefix.len() + r.0.len()
        }

        fn len_with(&self, other: Ref<'s>) -> usize {
            self.prefix.len() + other.0.len()
        }

        fn len_elided(&self, r: Ref<'_>) -> usize {
            self.prefix.len() + r.0.len()
        }
    }

    #[test]
    fn test_lifetimes_param_type() {
        let of_mock = LifetimesParamType::len_of_mock();
        of_mock.setup(|_, r: Ref<'_>| r.0.len());
        of_mock.expect_once();

        let with_mock = LifetimesParamType::len_with_mock();
        with_mock.setup(|_, r: Ref<'_>| r.0.len());
        with_mock.expect_once();

        let elided_mock = LifetimesParamType::len_elided_mock();
        elided_mock.setup(|_, r: Ref<'_>| r.0.len());
        elided_mock.expect_once();

        let s = LifetimesParamType { prefix: "ab" };
        assert_eq!(s.len_of(Ref("cde")), 3);
        assert_eq!(s.len_with(Ref("cde")), 3);
        assert_eq!(s.len_elided(Ref("cde")), 3);

        of_mock.assert();
        with_mock.assert();
        elided_mock.assert();
    }
}
