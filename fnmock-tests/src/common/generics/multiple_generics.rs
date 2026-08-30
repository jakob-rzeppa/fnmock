mod fake {
    #[fnmock::fakeable]
    fn multiple_generics<T: 'static, U: 'static>(a: T, b: U) -> (T, U) {
        (a, b)
    }

    #[test]
    fn test_multiple_generics() {
        let res = multiple_generics("Test".to_string(), "Another".to_string());
        assert_eq!(res, ("Test".to_string(), "Another".to_string()));
    }

    #[test]
    fn test_multiple_generics_fake() {
        // You should always specify the generic types when setting up a fake for a function with generics.
        // It is possible for the compiler to infer the types, but it is not guaranteed to work in all cases
        // and makes the code less readable.
        multiple_generics_fake::<String, String>()
            .setup(|a, b| (format!("Fake {}", a), format!("Fake {}", b)));
        let res = multiple_generics("Test".to_string(), "Another".to_string());
        assert_eq!(
            res,
            ("Fake Test".to_string(), "Fake Another".to_string())
        );
    }
}

mod spy {
    #[fnmock::spyable]
    fn multiple_generics<T: 'static, U: 'static>(a: T, b: U) -> (T, U) {
        (a, b)
    }

    #[test]
    fn test_multiple_generics() {
        let spy = multiple_generics_spy::<String, i32>();
        spy.expect(
            fnmock::predicate::eq("hi".to_string()),
            fnmock::predicate::eq(2),
        )
        .once();

        let res = multiple_generics("hi".to_string(), 2);

        assert_eq!(res, ("hi".to_string(), 2));
        spy.assert();
    }

    /// The store is keyed by the generic arguments as a whole, so swapping them
    /// around reaches a different instantiation entirely.
    #[test]
    fn test_swapped_generic_arguments_are_a_different_instantiation() {
        let spy_string_i32 = multiple_generics_spy::<String, i32>();
        let spy_i32_string = multiple_generics_spy::<i32, String>();
        spy_string_i32.expect_once();
        spy_i32_string.expect_never();

        multiple_generics("hi".to_string(), 2);

        spy_string_i32.assert();
        spy_i32_string.assert();
    }
}
