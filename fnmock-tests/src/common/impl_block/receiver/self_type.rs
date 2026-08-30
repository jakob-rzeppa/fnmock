mod fake {
    #[derive(Debug, PartialEq)]
    struct SelfType {
        value: i32,
    }

    #[fnmock::fakeable]
    impl SelfType {
        fn create(value: i32) -> Self {
            Self { value }
        }

        fn combine(&self, other: Self) -> Self {
            Self {
                value: self.value + other.value,
            }
        }
    }

    #[test]
    fn test_self_type() {
        let a = SelfType::create(42);
        let b = SelfType::create(8);
        assert_eq!(a.combine(b), SelfType { value: 50 });
    }

    #[test]
    fn test_self_type_fake() {
        SelfType::create_fake().setup(|value| SelfType { value: value * 2 });
        SelfType::combine_fake().setup(|_, _| SelfType { value: 100 });

        let a = SelfType::create(42);
        assert_eq!(a, SelfType { value: 84 });

        let b = SelfType::create(8);
        assert_eq!(a.combine(b), SelfType { value: 100 });
    }
}

mod spy {
    #[derive(Debug, PartialEq)]
    struct SelfType {
        value: i32,
    }

    #[fnmock::spyable]
    impl SelfType {
        fn create(value: i32) -> Self {
            Self { value }
        }

        fn combine(&self, other: Self) -> Self {
            Self {
                value: self.value + other.value,
            }
        }
    }

    #[test]
    fn test_self_type_spy() {
        let spy_create = SelfType::create_spy();
        let spy_combine = SelfType::combine_spy();
        spy_create.expect_times(2);
        spy_combine.expect_once();

        let a = SelfType::create(42);
        let b = SelfType::create(8);
        assert_eq!(a.combine(b), SelfType { value: 50 });

        spy_create.assert();
        spy_combine.assert();
    }
}
