mod fake {
    struct LifetimesBorrowedReturn<'s> {
        prefix: &'s str,
    }

    #[fnmock::fakeable]
    impl<'s> LifetimesBorrowedReturn<'s> {
        fn echo<'a>(&self, value: &'a str) -> &'a str {
            value
        }

        fn prefix(&self) -> &'s str {
            self.prefix
        }
    }

    #[test]
    fn test_lifetimes_borrowed_return() {
        let s = LifetimesBorrowedReturn { prefix: "Test" };
        assert_eq!(s.echo("Value"), "Value");
        assert_eq!(s.prefix(), "Test");
    }

    #[test]
    fn test_lifetimes_borrowed_return_fake() {
        LifetimesBorrowedReturn::echo_fake().setup(|_, value| value);
        LifetimesBorrowedReturn::prefix_fake().setup(|_| "Fake");

        let s = LifetimesBorrowedReturn { prefix: "Test" };
        assert_eq!(s.echo("Value"), "Value");
        assert_eq!(s.prefix(), "Fake");
    }
}

mod spy {
    struct LifetimesBorrowedReturn<'s> {
        prefix: &'s str,
    }

    #[fnmock::spyable]
    impl<'s> LifetimesBorrowedReturn<'s> {
        fn echo<'a>(&self, value: &'a str) -> &'a str {
            value
        }

        fn prefix(&self) -> &'s str {
            self.prefix
        }
    }

    #[test]
    fn test_lifetimes_borrowed_return_spy() {
        let echo_spy = LifetimesBorrowedReturn::echo_spy();
        let prefix_spy = LifetimesBorrowedReturn::prefix_spy();
        echo_spy.expect_once();
        prefix_spy.expect_once();

        let s = LifetimesBorrowedReturn { prefix: "Test" };
        assert_eq!(s.echo("Value"), "Value");
        assert_eq!(s.prefix(), "Test");

        echo_spy.assert();
        prefix_spy.assert();
    }
}

mod mock {
    struct LifetimesBorrowedReturn<'s> {
        prefix: &'s str,
    }

    #[fnmock::mockable]
    impl<'s> LifetimesBorrowedReturn<'s> {
        fn echo<'a>(&self, value: &'a str) -> &'a str {
            value
        }

        fn prefix(&self) -> &'s str {
            self.prefix
        }
    }

    #[test]
    fn test_lifetimes_borrowed_return() {
        let echo_mock = LifetimesBorrowedReturn::echo_mock();
        echo_mock.setup(|_, value| value);
        echo_mock.expect_once();

        let prefix_mock = LifetimesBorrowedReturn::prefix_mock();
        prefix_mock.setup(|_| "Fake");
        prefix_mock.expect_once();

        let s = LifetimesBorrowedReturn { prefix: "Test" };
        assert_eq!(s.echo("Value"), "Value");
        assert_eq!(s.prefix(), "Fake");

        echo_mock.assert();
        prefix_mock.assert();
    }
}
