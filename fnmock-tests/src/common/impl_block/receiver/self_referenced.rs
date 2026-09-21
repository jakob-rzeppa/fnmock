mod fake {
    struct SelfReferenced {
        value: i32,
    }

    #[fnmock::fakeable]
    impl SelfReferenced {
        fn get(&self) -> i32 {
            self.value
        }
    }

    #[test]
    fn test_self_referenced() {
        let s = SelfReferenced { value: 42 };
        assert_eq!(s.get(), 42);
    }

    #[test]
    fn test_self_referenced_fake() {
        SelfReferenced::get_fake().setup(|_| 5);

        let s = SelfReferenced { value: 42 };
        assert_eq!(s.get(), 5);
    }
}

mod spy {
    struct SelfReferenced {
        value: i32,
    }

    #[fnmock::spyable]
    impl SelfReferenced {
        fn get(&self) -> i32 {
            self.value
        }
    }

    #[test]
    fn test_self_referenced_spy() {
        let spy = SelfReferenced::get_spy();
        spy.expect_once();

        let s = SelfReferenced { value: 42 };
        assert_eq!(s.get(), 42);

        spy.assert();
    }
}
