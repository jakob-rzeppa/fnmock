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

mod mock {
    struct SelfMutReferenced {
        value: i32,
    }

    #[fnmock::mockable]
    impl SelfMutReferenced {
        fn increment(&mut self) -> i32 {
            self.value += 1;
            self.value
        }
    }

    #[test]
    fn test_self_mut_referenced() {
        let mock = SelfMutReferenced::increment_mock();
        mock.setup(|_| 5);
        mock.expect_once();

        let mut s = SelfMutReferenced { value: 42 };
        let res = s.increment();

        assert_eq!(res, 5);
        mock.assert();
    }
}
