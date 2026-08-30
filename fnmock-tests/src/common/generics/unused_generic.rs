mod fake {
    #[fnmock::fakeable]
    fn unused_generic<T: 'static>(id: i32) -> i32 {
        id
    }

    #[test]
    fn test_unused_generic() {
        let res = unused_generic::<String>(7);
        assert_eq!(res, 7);
    }

    #[test]
    fn test_unused_generic_fake() {
        unused_generic_fake::<String>().setup(|id| id + 1);
        let res = unused_generic::<String>(7);
        assert_eq!(res, 8);
    }
}

mod spy {
    #[fnmock::spyable]
    fn unused_generic<T: 'static>(id: i32) -> i32 {
        id
    }

    #[test]
    fn test_unused_generic() {
        let spy = unused_generic_spy::<String>();
        spy.expect(fnmock::predicate::eq(7)).once();

        let res = unused_generic::<String>(7);

        assert_eq!(res, 7);
        spy.assert();
    }

    #[test]
    fn test_unused_generic_still_keys_the_store() {
        let spy_string = unused_generic_spy::<String>();
        let spy_bool = unused_generic_spy::<bool>();
        spy_string.expect_once();
        spy_bool.expect_never();

        unused_generic::<String>(7);

        spy_string.assert();
        spy_bool.assert();
    }
}
