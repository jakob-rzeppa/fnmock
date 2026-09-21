mod fake {
    struct SelfBoxed {
        value: i32,
    }

    #[fnmock::fakeable]
    impl SelfBoxed {
        fn get(self: Box<Self>) -> i32 {
            self.value
        }
    }

    #[test]
    fn test_self_boxed() {
        let s = Box::new(SelfBoxed { value: 42 });
        assert_eq!(s.get(), 42);
    }

    #[test]
    fn test_self_boxed_fake() {
        SelfBoxed::get_fake().setup(|_| 5);

        let s = Box::new(SelfBoxed { value: 42 });
        assert_eq!(s.get(), 5);
    }
}

mod spy {
    struct SelfBoxed {
        value: i32,
    }

    #[fnmock::spyable]
    impl SelfBoxed {
        fn get(self: Box<Self>) -> i32 {
            self.value
        }
    }

    #[test]
    fn test_self_boxed_spy() {
        let spy = SelfBoxed::get_spy();
        spy.expect_once();

        let s = Box::new(SelfBoxed { value: 42 });
        assert_eq!(s.get(), 42);

        spy.assert();
    }
}
