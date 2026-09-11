mod fake {
    #[fnmock::fakeable]
    fn unused_lifetime<'a>(id: i32) -> i32 {
        id
    }

    #[test]
    fn test_unused_lifetime() {
        let res = unused_lifetime(7);
        assert_eq!(res, 7);
    }

    #[test]
    fn test_unused_lifetime_fake() {
        unused_lifetime_fake().setup(|id| id + 1);

        let res = unused_lifetime(7);
        assert_eq!(res, 8);
    }
}

mod spy {
    #[fnmock::spyable]
    fn unused_lifetime<'a>(id: i32) -> i32 {
        id
    }

    #[test]
    fn test_unused_lifetime() {
        let spy = unused_lifetime_spy();
        spy.expect(fnmock::predicate::eq(7)).once();

        let res = unused_lifetime(7);

        assert_eq!(res, 7);
        spy.assert();
    }
}
