mod fake {
    struct GenericMethodWhere;

    #[fnmock::fakeable]
    impl GenericMethodWhere {
        fn echo<T>(&self, a: T) -> T
        where
            T: 'static,
        {
            a
        }
    }

    #[test]
    fn test_generic_method_where() {
        let s = GenericMethodWhere;
        assert_eq!(s.echo("Test".to_string()), "Test");
    }

    #[test]
    fn test_generic_method_where_fake() {
        GenericMethodWhere::echo_fake::<String>().setup(|_, a| format!("Fake {}", a));

        let s = GenericMethodWhere;
        assert_eq!(s.echo("Test".to_string()), "Fake Test");
    }
}

mod spy {
    struct GenericMethodWhere;

    #[fnmock::spyable]
    impl GenericMethodWhere {
        fn echo<T>(&self, a: T) -> T
        where
            T: 'static,
        {
            a
        }
    }

    #[test]
    fn test_generic_method_where_spy() {
        let spy = GenericMethodWhere::echo_spy::<String>();
        spy.expect_once();

        let s = GenericMethodWhere;
        assert_eq!(s.echo("Test".to_string()), "Test");

        spy.assert();
    }
}

mod mock {
    struct GenericMethodWhere;

    #[fnmock::mockable]
    impl GenericMethodWhere {
        fn echo<T>(&self, a: T) -> T
        where
            T: 'static,
        {
            a
        }
    }

    #[test]
    fn test_generic_method_where() {
        let mock = GenericMethodWhere::echo_mock::<String>();
        mock.setup(|_, a| format!("Fake {}", a));
        mock.expect(fnmock::predicate::eq("Test".to_string()))
            .once();

        let s = GenericMethodWhere;
        let res = s.echo("Test".to_string());

        assert_eq!(res, "Fake Test");
        mock.assert();
    }
}
