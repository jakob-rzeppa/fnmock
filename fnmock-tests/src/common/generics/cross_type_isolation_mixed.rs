mod fake {
    #[fnmock::fakeable]
    fn cross_type_isolation_mixed<T: 'static, const C: usize>(a: T) -> usize {
        let _ = a;
        C
    }

    #[test]
    fn test_cross_type_isolation_mixed() {
        let res = cross_type_isolation_mixed::<String, 5>("Test".to_string());
        assert_eq!(res, 5);
    }

    #[test]
    fn test_cross_type_isolation_mixed_fake() {
        cross_type_isolation_mixed_fake::<String, 5>().setup(|_a| 99);

        let res = cross_type_isolation_mixed::<String, 5>("Test".to_string());
        assert_eq!(res, 99);

        // A different const value falls back to the real implementation.
        let res = cross_type_isolation_mixed::<String, 7>("Test".to_string());
        assert_eq!(res, 7);

        // A different type parameter also falls back to the real implementation, even though
        // the const value matches.
        let res = cross_type_isolation_mixed::<i32, 5>(9);
        assert_eq!(res, 5);

        // The fake for (String, 5) should remain unaffected.
        let res = cross_type_isolation_mixed::<String, 5>("Test".to_string());
        assert_eq!(res, 99);
    }
}

mod spy {
    #[fnmock::spyable]
    fn cross_type_isolation_mixed<T: 'static, const C: usize>(a: T) -> usize {
        let _ = a;
        C
    }

    #[test]
    fn test_cross_type_isolation_mixed() {
        let spy = cross_type_isolation_mixed_spy::<String, 4>();
        spy.expect(fnmock::predicate::eq("hi".to_string())).once();

        let res = cross_type_isolation_mixed::<String, 4>("hi".to_string());

        assert_eq!(res, 4);
        spy.assert();
    }

    #[test]
    fn test_expectations_do_not_leak_across_instantiations() {
        let spy_string_4 = cross_type_isolation_mixed_spy::<String, 4>();
        let spy_string_8 = cross_type_isolation_mixed_spy::<String, 8>();
        let spy_i32_4 = cross_type_isolation_mixed_spy::<i32, 4>();
        spy_string_4.expect_once();
        spy_string_8.expect_never();
        spy_i32_4.expect_never();

        cross_type_isolation_mixed::<String, 4>("hi".to_string());

        spy_string_4.assert();
        spy_string_8.assert();
        spy_i32_4.assert();
    }
}
