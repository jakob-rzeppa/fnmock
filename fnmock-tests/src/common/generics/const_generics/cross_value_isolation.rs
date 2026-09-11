mod fake {
    #[fnmock::fakeable]
    fn cross_value_isolation<const C: usize>(a: String) -> String {
        format!("{} {}", a, C)
    }

    #[test]
    fn test_cross_value_isolation_fake_does_not_leak_across_values() {
        cross_value_isolation_fake::<5>().setup(|a| format!("Fake {} {}", a, 5));

        // A fake was only set up for C=5, so a call with a different value of C
        // must still run the real implementation.
        let res = cross_value_isolation::<7>("Test".to_string());
        assert_eq!(res, "Test 7");

        // The fake for C=5 should remain unaffected.
        let res = cross_value_isolation::<5>("Test".to_string());
        assert_eq!(res, "Fake Test 5");
    }
}

mod spy {
    //! Const parameters are keyed by their value, not by the `TypeId` of their
    //! type, so `C = 5` and `C = 7` are separate instantiations.

    #[fnmock::spyable]
    fn cross_value_isolation<const C: usize>(a: String) -> String {
        format!("{} {}", a, C)
    }

    #[test]
    fn test_expectations_do_not_leak_across_values() {
        let spy_5 = cross_value_isolation_spy::<5>();
        let spy_7 = cross_value_isolation_spy::<7>();
        spy_5.expect_once();
        spy_7.expect_never();

        cross_value_isolation::<5>("Test".to_string());

        spy_5.assert();
        spy_7.assert();
    }
}
