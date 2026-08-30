mod fake {
    struct AssociatedFunction;

    #[fnmock::fakeable]
    impl AssociatedFunction {
        fn associated(a: i32) -> i32 {
            a
        }
    }

    #[test]
    fn test_associated_function() {
        assert_eq!(AssociatedFunction::associated(42), 42);
    }

    #[test]
    fn test_associated_function_fake() {
        AssociatedFunction::associated_fake().setup(|_| 5);

        assert_eq!(AssociatedFunction::associated(42), 5);
    }
}

mod spy {
    struct AssociatedFunction;

    #[fnmock::spyable]
    impl AssociatedFunction {
        fn associated(a: i32) -> i32 {
            a
        }
    }

    #[test]
    fn test_associated_function_spy() {
        let spy = AssociatedFunction::associated_spy();
        spy.expect_once();

        assert_eq!(AssociatedFunction::associated(42), 42);

        spy.assert();
    }
}
