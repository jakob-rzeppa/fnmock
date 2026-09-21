mod fake {
    #[fnmock::fakeable]
    fn return_generic<T: Default + 'static>() -> T {
        T::default()
    }

    #[test]
    fn test_return_generic() {
        let res = return_generic::<String>();
        assert_eq!(res, String::default());
    }

    #[test]
    fn test_return_generic_fake() {
        return_generic_fake::<String>().setup(|| "Fake".to_string());

        let res = return_generic::<String>();
        assert_eq!(res, "Fake".to_string());
    }
}

mod spy {
    #[fnmock::spyable]
    fn return_generic<T: Default + 'static>() -> T {
        T::default()
    }

    #[test]
    fn test_return_generic() {
        let spy = return_generic_spy::<String>();
        spy.expect_once();

        let res = return_generic::<String>();

        assert_eq!(res, String::default());
        spy.assert();
    }

    #[test]
    fn test_return_generic_is_keyed_by_instantiation() {
        let spy_string = return_generic_spy::<String>();
        let spy_i32 = return_generic_spy::<i32>();
        spy_string.expect_once();
        spy_i32.expect_never();

        return_generic::<String>();

        spy_string.assert();
        spy_i32.assert();
    }
}
