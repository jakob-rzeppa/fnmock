mod fake {
    struct StaticBoundViaWhereLifetime<T> {
        value: T,
    }

    #[fnmock::fakeable]
    impl<'a, T: 'a + Clone> StaticBoundViaWhereLifetime<T>
    where
        'a: 'static,
    {
        fn get(&self) -> T {
            self.value.clone()
        }
    }

    #[test]
    fn test_static_bound_via_where_lifetime() {
        let s = StaticBoundViaWhereLifetime {
            value: "Test".to_string(),
        };
        assert_eq!(s.get(), "Test");
    }

    #[test]
    fn test_static_bound_via_where_lifetime_fake() {
        StaticBoundViaWhereLifetime::<String>::get_fake().setup(|_| "Fake".to_string());

        let s = StaticBoundViaWhereLifetime {
            value: "Test".to_string(),
        };
        assert_eq!(s.get(), "Fake");
    }
}

mod spy {
    struct StaticBoundViaWhereLifetime<T> {
        value: T,
    }

    #[fnmock::spyable]
    impl<'a, T: 'a + Clone> StaticBoundViaWhereLifetime<T>
    where
        'a: 'static,
    {
        fn get(&self) -> T {
            self.value.clone()
        }
    }

    #[test]
    fn test_static_bound_via_where_lifetime_spy() {
        let spy = StaticBoundViaWhereLifetime::<String>::get_spy();
        spy.expect_once();

        let s = StaticBoundViaWhereLifetime {
            value: "Test".to_string(),
        };
        assert_eq!(s.get(), "Test");

        spy.assert();
    }
}

mod mock {
    struct StaticBoundViaWhereLifetime<T> {
        value: T,
    }

    #[fnmock::mockable]
    impl<'a, T: 'a + Clone> StaticBoundViaWhereLifetime<T>
    where
        'a: 'static,
    {
        fn get(&self) -> T {
            self.value.clone()
        }
    }

    #[test]
    fn test_static_bound_via_where_lifetime() {
        let mock = StaticBoundViaWhereLifetime::<String>::get_mock();
        mock.setup(|_| "Fake".to_string());
        mock.expect_once();

        let s = StaticBoundViaWhereLifetime {
            value: "Test".to_string(),
        };
        let res = s.get();

        assert_eq!(res, "Fake");
        mock.assert();
    }
}
