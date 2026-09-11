mod fake {
    #[fnmock::fakeable]
    fn unused_const_generic<const N: usize>(id: i32) -> i32 {
        id
    }

    #[test]
    fn test_unused_const_generic() {
        let res = unused_const_generic::<3>(7);
        assert_eq!(res, 7);
    }

    #[test]
    fn test_unused_const_generic_fake() {
        unused_const_generic_fake::<3>().setup(|id| id + 1);
        let res = unused_const_generic::<3>(7);
        assert_eq!(res, 8);
    }
}

mod spy {
    #[fnmock::spyable]
    fn unused_const_generic<const N: usize>(id: i32) -> i32 {
        id
    }

    #[test]
    fn test_unused_const_generic() {
        let spy = unused_const_generic_spy::<3>();
        spy.expect(fnmock::predicate::eq(7)).once();

        let res = unused_const_generic::<3>(7);

        assert_eq!(res, 7);
        spy.assert();
    }

    #[test]
    fn test_unused_const_generic_still_keys_the_store() {
        let spy_3 = unused_const_generic_spy::<3>();
        let spy_5 = unused_const_generic_spy::<5>();
        spy_3.expect_once();
        spy_5.expect_never();

        unused_const_generic::<3>(7);

        spy_3.assert();
        spy_5.assert();
    }
}
