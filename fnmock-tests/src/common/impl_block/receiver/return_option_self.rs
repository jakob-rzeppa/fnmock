mod fake {
    #[derive(Debug, PartialEq)]
    struct ReturnOptionSelf {
        value: i32,
    }

    #[fnmock::fakeable]
    impl ReturnOptionSelf {
        fn checked(&self) -> Option<Self> {
            if self.value > 0 {
                Some(Self { value: self.value })
            } else {
                None
            }
        }
    }

    #[test]
    fn test_return_option_self() {
        let s = ReturnOptionSelf { value: 42 };
        assert_eq!(s.checked(), Some(ReturnOptionSelf { value: 42 }));
    }

    #[test]
    fn test_return_option_self_fake() {
        ReturnOptionSelf::checked_fake().setup(|_| None);

        let s = ReturnOptionSelf { value: 42 };
        assert_eq!(s.checked(), None);
    }
}

mod spy {
    #[derive(Debug, PartialEq)]
    struct ReturnOptionSelf {
        value: i32,
    }

    #[fnmock::spyable]
    impl ReturnOptionSelf {
        fn checked(&self) -> Option<Self> {
            if self.value > 0 {
                Some(Self { value: self.value })
            } else {
                None
            }
        }
    }

    #[test]
    fn test_return_option_self_spy() {
        let spy = ReturnOptionSelf::checked_spy();
        spy.expect_once();

        let s = ReturnOptionSelf { value: 42 };
        assert_eq!(s.checked(), Some(ReturnOptionSelf { value: 42 }));

        spy.assert();
    }
}
