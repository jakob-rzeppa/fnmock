mod fake {
    struct GenericCombinedWhere<T> {
        value: T,
    }

    #[fnmock::fakeable]
    impl<T> GenericCombinedWhere<T>
    where
        T: Clone + 'static,
    {
        fn combine<U>(&self, other: U) -> (T, U)
        where
            U: 'static,
        {
            (self.value.clone(), other)
        }
    }

    #[test]
    fn test_generic_combined_where() {
        let s = GenericCombinedWhere {
            value: "Test".to_string(),
        };
        assert_eq!(s.combine(42), ("Test".to_string(), 42));
    }

    #[test]
    fn test_generic_combined_where_fake() {
        GenericCombinedWhere::<String>::combine_fake::<i32>()
            .setup(|_, other| ("Fake".to_string(), other * 2));

        let s = GenericCombinedWhere {
            value: "Test".to_string(),
        };
        assert_eq!(s.combine(42), ("Fake".to_string(), 84));
    }
}

mod spy {
    struct GenericCombinedWhere<T> {
        value: T,
    }

    #[fnmock::spyable]
    impl<T> GenericCombinedWhere<T>
    where
        T: Clone + 'static,
    {
        fn combine<U>(&self, other: U) -> (T, U)
        where
            U: 'static,
        {
            (self.value.clone(), other)
        }
    }

    #[test]
    fn test_generic_combined_where_spy() {
        let spy = GenericCombinedWhere::<String>::combine_spy::<i32>();
        spy.expect_once();

        let s = GenericCombinedWhere {
            value: "Test".to_string(),
        };
        assert_eq!(s.combine(42), ("Test".to_string(), 42));

        spy.assert();
    }
}

mod mock {
    struct GenericCombinedWhere<T> {
        value: T,
    }

    #[fnmock::mockable]
    impl<T> GenericCombinedWhere<T>
    where
        T: Clone + 'static,
    {
        fn combine<U>(&self, other: U) -> (T, U)
        where
            U: 'static,
        {
            (self.value.clone(), other)
        }
    }

    #[test]
    fn test_generic_combined_where() {
        let mock = GenericCombinedWhere::<String>::combine_mock::<i32>();
        mock.setup(|_, other| ("Fake".to_string(), other * 2));
        mock.expect_once();

        let s = GenericCombinedWhere {
            value: "Test".to_string(),
        };
        let res = s.combine(42);

        assert_eq!(res, ("Fake".to_string(), 84));
        mock.assert();
    }
}
