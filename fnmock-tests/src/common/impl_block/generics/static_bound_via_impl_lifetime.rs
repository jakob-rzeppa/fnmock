mod fake {
    struct StaticBoundViaImplLifetime;

    #[fnmock::fakeable]
    impl<'a> StaticBoundViaImplLifetime
    where
        'a: 'static,
    {
        fn show<T: 'a + std::fmt::Display>(&self, value: T) -> String {
            format!("{value}")
        }
    }

    #[test]
    fn test_static_bound_via_impl_lifetime() {
        let s = StaticBoundViaImplLifetime;
        assert_eq!(s.show("Test".to_string()), "Test");
    }

    #[test]
    fn test_static_bound_via_impl_lifetime_fake() {
        StaticBoundViaImplLifetime::show_fake::<String>().setup(|_, value| format!("Fake {value}"));

        let s = StaticBoundViaImplLifetime;
        assert_eq!(s.show("Test".to_string()), "Fake Test");
    }
}

mod spy {
    struct StaticBoundViaImplLifetime;

    #[fnmock::spyable]
    impl<'a> StaticBoundViaImplLifetime
    where
        'a: 'static,
    {
        fn show<T: 'a + std::fmt::Display>(&self, value: T) -> String {
            format!("{value}")
        }
    }

    #[test]
    fn test_static_bound_via_impl_lifetime_spy() {
        let spy = StaticBoundViaImplLifetime::show_spy::<String>();
        spy.expect_once();

        let s = StaticBoundViaImplLifetime;
        assert_eq!(s.show("Test".to_string()), "Test");

        spy.assert();
    }
}
