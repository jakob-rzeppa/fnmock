mod fake {
    struct GenericMethod;

    #[fnmock::fakeable]
    impl GenericMethod {
        fn echo<T: 'static>(&self, a: T) -> T {
            a
        }
    }

    #[test]
    fn test_generic_method() {
        let s = GenericMethod;
        assert_eq!(s.echo("Test".to_string()), "Test");
    }

    #[test]
    fn test_generic_method_fake() {
        GenericMethod::echo_fake::<String>().setup(|_, a| format!("Fake {}", a));

        let s = GenericMethod;
        assert_eq!(s.echo("Test".to_string()), "Fake Test");
    }
}

mod spy {
    struct GenericMethod;

    #[fnmock::spyable]
    impl GenericMethod {
        fn echo<T: 'static>(&self, a: T) -> T {
            a
        }
    }

    #[test]
    fn test_generic_method_spy() {
        let spy = GenericMethod::echo_spy::<String>();
        spy.expect_once();

        let s = GenericMethod;
        assert_eq!(s.echo("Test".to_string()), "Test");

        spy.assert();
    }
}

mod mock {
    struct GenericMethod;

    #[fnmock::mockable]
    impl GenericMethod {
        fn echo<T: 'static>(&self, a: T) -> T {
            a
        }
    }

    #[test]
    fn test_generic_method() {
        let mock = GenericMethod::echo_mock::<String>();
        mock.setup(|_, a| format!("Fake {}", a));
        mock.expect(fnmock::predicate::eq("Test".to_string()))
            .once();

        let s = GenericMethod;
        let res = s.echo("Test".to_string());

        assert_eq!(res, "Fake Test");
        mock.assert();
    }
}
