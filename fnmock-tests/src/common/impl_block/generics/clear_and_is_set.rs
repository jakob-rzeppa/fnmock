mod fake {
    //! `clear()` / `is_set()` on a generic impl-block method, which goes through
    //! `GenericFakeStore` keyed by the method's generic parameters.

    struct GenericClearAndIsSet;

    #[fnmock::fakeable]
    impl GenericClearAndIsSet {
        fn echo<T: 'static + std::fmt::Display>(&self, a: T) -> String {
            format!("Real {}", a)
        }
    }

    #[test]
    fn test_is_set_transitions() {
        assert!(!GenericClearAndIsSet::echo_fake::<String>().is_set());

        GenericClearAndIsSet::echo_fake::<String>().setup(|_, a| format!("Fake {}", a));
        assert!(GenericClearAndIsSet::echo_fake::<String>().is_set());

        GenericClearAndIsSet::echo_fake::<String>().clear();
        assert!(!GenericClearAndIsSet::echo_fake::<String>().is_set());
    }

    #[test]
    fn test_clear_one_instantiation_leaves_others_installed() {
        let s = GenericClearAndIsSet;

        GenericClearAndIsSet::echo_fake::<String>().setup(|_, a| format!("Fake {}", a));
        GenericClearAndIsSet::echo_fake::<i32>().setup(|_, a| format!("Faked {}", a));

        GenericClearAndIsSet::echo_fake::<String>().clear();

        // Clearing the String instantiation must leave the i32 one untouched.
        assert!(!GenericClearAndIsSet::echo_fake::<String>().is_set());
        assert!(GenericClearAndIsSet::echo_fake::<i32>().is_set());

        assert_eq!(s.echo("Test".to_string()), "Real Test");
        assert_eq!(s.echo(42), "Faked 42");
    }
}

mod spy {
    //! Spy expectations on a generic impl-block method are likewise keyed by the method's
    //! generic parameters.

    struct GenericClearAndIsSet;

    #[fnmock::spyable]
    impl GenericClearAndIsSet {
        fn echo<T: 'static + std::fmt::Display>(&self, a: T) -> String {
            format!("Real {}", a)
        }
    }

    #[test]
    fn test_instantiations_stay_isolated() {
        let s = GenericClearAndIsSet;

        let spy_string = GenericClearAndIsSet::echo_spy::<String>();
        let spy_i32 = GenericClearAndIsSet::echo_spy::<i32>();
        spy_string.expect_once();
        spy_i32.expect_never();

        assert_eq!(s.echo("Test".to_string()), "Real Test");

        spy_string.assert();
        spy_i32.assert();
    }
}
