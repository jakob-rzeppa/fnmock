mod fake {
    struct SelfConsumed {
        value: i32,
    }

    #[fnmock::fakeable]
    impl SelfConsumed {
        fn into_value(self) -> i32 {
            self.value
        }
    }

    #[test]
    fn test_self_consumed() {
        let s = SelfConsumed { value: 42 };
        assert_eq!(s.into_value(), 42);
    }

    #[test]
    fn test_self_consumed_fake() {
        SelfConsumed::into_value_fake().setup(|_| 5);

        let s = SelfConsumed { value: 42 };
        assert_eq!(s.into_value(), 5);
    }
}

mod spy {
    struct SelfConsumed {
        value: i32,
    }

    #[fnmock::spyable]
    impl SelfConsumed {
        fn into_value(self) -> i32 {
            self.value
        }
    }

    #[test]
    fn test_self_consumed_spy() {
        let spy = SelfConsumed::into_value_spy();
        spy.expect_once();

        let s = SelfConsumed { value: 42 };
        assert_eq!(s.into_value(), 42);

        spy.assert();
    }
}
