//! Doc comments (`///`, lowered to `#[doc = "..."]`) on the function must not
//! confuse fnmock expansion; the fake / spy accessors still work.

mod fake {
    /// Before
    #[fnmock::fakeable]
    /// After
    fn add_one(value: i32) -> i32 {
        value + 1
    }

    #[test]
    fn test_add_one() {
        assert_eq!(add_one(1), 2);
    }

    #[test]
    fn test_add_one_fake() {
        add_one_fake().setup(|value| value + 10);

        assert_eq!(add_one(1), 11);
    }
}

mod spy {
    /// Before
    #[fnmock::spyable]
    /// After
    fn add_one(value: i32) -> i32 {
        value + 1
    }

    #[test]
    fn test_add_one_spy() {
        let spy = add_one_spy();
        spy.expect(fnmock::predicate::eq(1));

        assert_eq!(add_one(1), 2);

        spy.assert();
    }
}
