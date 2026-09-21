mod fake {
    #[fnmock::fakeable]
    fn generic_clear_and_is_set<T: 'static + std::fmt::Display>(a: T) -> String {
        format!("Real {}", a)
    }

    #[test]
    fn test_is_set_transitions() {
        assert!(!generic_clear_and_is_set_fake::<String>().is_set());

        generic_clear_and_is_set_fake::<String>().setup(|a| format!("Fake {}", a));
        assert!(generic_clear_and_is_set_fake::<String>().is_set());

        generic_clear_and_is_set_fake::<String>().clear();
        assert!(!generic_clear_and_is_set_fake::<String>().is_set());
    }

    #[test]
    fn test_is_set_is_scoped_to_one_instantiation() {
        generic_clear_and_is_set_fake::<String>().setup(|a| format!("Fake {}", a));

        assert!(generic_clear_and_is_set_fake::<String>().is_set());
        assert!(!generic_clear_and_is_set_fake::<i32>().is_set());
    }

    #[test]
    fn test_clear_one_instantiation_leaves_others_installed() {
        generic_clear_and_is_set_fake::<String>().setup(|a| format!("Fake {}", a));
        generic_clear_and_is_set_fake::<i32>().setup(|a| format!("Faked {}", a));

        generic_clear_and_is_set_fake::<String>().clear();

        // Clearing the String instantiation must leave the i32 one untouched.
        assert!(!generic_clear_and_is_set_fake::<String>().is_set());
        assert!(generic_clear_and_is_set_fake::<i32>().is_set());

        assert_eq!(generic_clear_and_is_set("Test".to_string()), "Real Test");
        assert_eq!(generic_clear_and_is_set(42), "Faked 42");
    }

    #[test]
    fn test_clear_on_an_instantiation_that_was_never_set_is_a_no_op() {
        generic_clear_and_is_set_fake::<u8>().clear();

        assert!(!generic_clear_and_is_set_fake::<u8>().is_set());
        assert_eq!(generic_clear_and_is_set(7u8), "Real 7");
    }
}
