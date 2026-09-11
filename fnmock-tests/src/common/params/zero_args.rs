mod fake {
    #[fnmock::fakeable]
    fn zero_args() -> i32 {
        1
    }

    #[test]
    fn test_zero_args() {
        let res = zero_args();
        assert_eq!(res, 1);
    }

    #[test]
    fn test_zero_args_fake() {
        zero_args_fake().setup(|| 42);
        let res = zero_args();
        assert_eq!(res, 42);
    }
}

mod spy {
    #[fnmock::spyable]
    fn no_params() {}

    #[test]
    fn test_no_params() {
        let spy = no_params_spy();
        spy.expect_times(1);
        spy.expect().times(1);

        no_params();

        spy.assert();
    }
}
