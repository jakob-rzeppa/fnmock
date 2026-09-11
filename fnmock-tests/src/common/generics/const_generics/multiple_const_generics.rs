mod fake {
    #[fnmock::fakeable]
    fn multiple_const_generics<const A: usize, const B: usize>(value: String) -> String {
        format!("{} {} {}", value, A, B)
    }

    #[test]
    fn test_multiple_const_generics() {
        let res = multiple_const_generics::<3, 5>("Test".to_string());
        assert_eq!(res, "Test 3 5");
    }

    #[test]
    fn test_multiple_const_generics_fake() {
        multiple_const_generics_fake::<3, 5>().setup(|value| format!("Fake {} 3 5", value));

        let res = multiple_const_generics::<3, 5>("Test".to_string());
        assert_eq!(res, "Fake Test 3 5");
    }

    #[test]
    fn test_multiple_const_generics_value_isolation() {
        multiple_const_generics_fake::<3, 5>().setup(|value| format!("Fake {}", value));

        // A fake was only set up for A=3, B=5, so a call with a different value for either
        // const parameter must still run the real implementation.
        let res = multiple_const_generics::<3, 7>("Test".to_string());
        assert_eq!(res, "Test 3 7");

        let res = multiple_const_generics::<4, 5>("Test".to_string());
        assert_eq!(res, "Test 4 5");

        // The fake for A=3, B=5 should remain unaffected.
        let res = multiple_const_generics::<3, 5>("Test".to_string());
        assert_eq!(res, "Fake Test");
    }
}

mod spy {
    #[fnmock::spyable]
    fn multiple_const_generics<const A: usize, const B: usize>(tag: &str) -> usize {
        let _ = tag;
        A + B
    }

    #[test]
    fn test_multiple_const_generics() {
        let spy = multiple_const_generics_spy::<2, 3>();
        spy.expect(fnmock::predicate::eq("tag".to_string())).once();

        let res = multiple_const_generics::<2, 3>("tag");

        assert_eq!(res, 5);
        spy.assert();
    }

    #[test]
    fn test_multiple_const_generics_are_keyed_positionally() {
        let spy_2_3 = multiple_const_generics_spy::<2, 3>();
        let spy_3_2 = multiple_const_generics_spy::<3, 2>();
        spy_2_3.expect_once();
        spy_3_2.expect_never();

        multiple_const_generics::<2, 3>("tag");

        spy_2_3.assert();
        spy_3_2.assert();
    }
}
