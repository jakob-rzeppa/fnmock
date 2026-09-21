mod fake {
    #[derive(Debug, PartialEq)]
    struct ReturnResultSelf {
        value: i32,
    }

    #[fnmock::fakeable]
    impl ReturnResultSelf {
        fn validated(&self) -> Result<Self, String> {
            if self.value > 0 {
                Ok(Self { value: self.value })
            } else {
                Err("invalid".to_string())
            }
        }
    }

    #[test]
    fn test_return_result_self() {
        let s = ReturnResultSelf { value: 42 };
        assert_eq!(s.validated(), Ok(ReturnResultSelf { value: 42 }));
    }

    #[test]
    fn test_return_result_self_fake() {
        ReturnResultSelf::validated_fake().setup(|_| Err("faked".to_string()));

        let s = ReturnResultSelf { value: 42 };
        assert_eq!(s.validated(), Err("faked".to_string()));
    }
}

mod spy {
    #[derive(Debug, PartialEq)]
    struct ReturnResultSelf {
        value: i32,
    }

    #[fnmock::spyable]
    impl ReturnResultSelf {
        fn validated(&self) -> Result<Self, String> {
            if self.value > 0 {
                Ok(Self { value: self.value })
            } else {
                Err("invalid".to_string())
            }
        }
    }

    #[test]
    fn test_return_result_self_spy() {
        let spy = ReturnResultSelf::validated_spy();
        spy.expect_once();

        let s = ReturnResultSelf { value: 42 };
        assert_eq!(s.validated(), Ok(ReturnResultSelf { value: 42 }));

        spy.assert();
    }
}
