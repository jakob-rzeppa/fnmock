//! A body written on one line (`{ a + 1 }`) must not trigger `unused_braces`, which would break
//! users building with `-D warnings`. rustfmt expands such bodies to multiple lines, hiding the
//! lint, hence the `#[rustfmt::skip]`s. Empty bodies (`{}`) are covered too.

#![deny(unused_braces)]

mod fake {
    #[rustfmt::skip]
    #[fnmock::fakeable]
    fn add_one(value: i32) -> i32 { value + 1 }

    #[rustfmt::skip]
    #[fnmock::fakeable]
    fn noop() {}

    struct Counter;

    #[rustfmt::skip]
    #[fnmock::fakeable]
    impl Counter {
        fn add_one(&self, value: i32) -> i32 { value + 1 }
    }

    #[test]
    fn test_add_one() {
        assert_eq!(add_one(1), 2);
        noop();
        assert_eq!(Counter.add_one(1), 2);
    }

    #[test]
    fn test_add_one_fake() {
        add_one_fake().setup(|value| value + 10);
        Counter::add_one_fake().setup(|_, value| value + 10);

        assert_eq!(add_one(1), 11);
        assert_eq!(Counter.add_one(1), 11);
    }
}

mod spy {
    #[rustfmt::skip]
    #[fnmock::spyable]
    fn add_one(value: i32) -> i32 { value + 1 }

    #[rustfmt::skip]
    #[fnmock::spyable]
    fn noop() {}

    struct Counter;

    #[rustfmt::skip]
    #[fnmock::spyable]
    impl Counter {
        fn add_one(&self, value: i32) -> i32 { value + 1 }
    }

    #[test]
    fn test_add_one_spy() {
        let spy = add_one_spy();
        spy.expect(fnmock::predicate::eq(1)).once();
        let noop_spy = noop_spy();
        noop_spy.expect_once();
        let method_spy = Counter::add_one_spy();
        method_spy.expect(fnmock::predicate::eq(1)).once();

        assert_eq!(add_one(1), 2);
        noop();
        assert_eq!(Counter.add_one(1), 2);

        spy.assert();
        noop_spy.assert();
        method_spy.assert();
    }
}

mod mock {
    #[rustfmt::skip]
    #[fnmock::mockable]
    fn add_one(value: i32) -> i32 { value + 1 }

    #[rustfmt::skip]
    #[fnmock::mockable]
    fn noop() {}

    struct Counter;

    #[rustfmt::skip]
    #[fnmock::mockable]
    impl Counter {
        fn add_one(&self, value: i32) -> i32 { value + 1 }
    }

    #[test]
    fn test_add_one_mock() {
        let mock = add_one_mock();
        mock.setup(|value| value + 10);
        mock.expect(fnmock::predicate::eq(1)).once();
        let noop_mock = noop_mock();
        noop_mock.expect_once();
        let method_mock = Counter::add_one_mock();
        method_mock.setup(|_, value| value + 10);
        method_mock.expect(fnmock::predicate::eq(1)).once();

        assert_eq!(add_one(1), 11);
        noop();
        assert_eq!(Counter.add_one(1), 11);

        mock.assert();
        noop_mock.assert();
        method_mock.assert();
    }
}
