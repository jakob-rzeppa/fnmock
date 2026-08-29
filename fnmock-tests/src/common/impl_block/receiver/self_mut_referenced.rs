mod fake {
    struct SelfMutReferenced {
        value: i32,
    }

    #[fnmock::fakeable]
    impl SelfMutReferenced {
        fn increment(&mut self) -> i32 {
            self.value += 1;
            self.value
        }
    }

    #[test]
    fn test_self_mut_referenced() {
        let mut s = SelfMutReferenced { value: 42 };
        assert_eq!(s.increment(), 43);
    }

    #[test]
    fn test_self_mut_referenced_fake() {
        SelfMutReferenced::increment_fake().setup(|_| 5);

        let mut s = SelfMutReferenced { value: 42 };
        assert_eq!(s.increment(), 5);
    }
}

mod spy {
    struct SelfMutReferenced {
        value: i32,
    }

    #[fnmock::spyable]
    impl SelfMutReferenced {
        fn increment(&mut self) -> i32 {
            self.value += 1;
            self.value
        }
    }

    #[test]
    fn test_self_mut_referenced_spy() {
        let spy = SelfMutReferenced::increment_spy();
        spy.expect_once();

        let mut s = SelfMutReferenced { value: 42 };
        assert_eq!(s.increment(), 43);

        spy.assert();
    }
}
