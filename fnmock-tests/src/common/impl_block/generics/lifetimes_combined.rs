mod fake {
    struct LifetimesCombined<'s> {
        prefix: &'s str,
    }

    #[fnmock::fakeable]
    impl<'s> LifetimesCombined<'s> {
        fn describe<'a>(&self, suffix: &'a str) -> String {
            format!("{}{}", self.prefix, suffix)
        }
    }

    #[test]
    fn test_lifetimes_combined() {
        let s = LifetimesCombined { prefix: "Test" };
        assert_eq!(s.describe(" Value"), "Test Value");
    }

    #[test]
    fn test_lifetimes_combined_fake() {
        LifetimesCombined::describe_fake().setup(|_, suffix| format!("Fake{}", suffix));

        let s = LifetimesCombined { prefix: "Test" };
        assert_eq!(s.describe(" Value"), "Fake Value");
    }
}

mod spy {
    struct LifetimesCombined<'s> {
        prefix: &'s str,
    }

    #[fnmock::spyable]
    impl<'s> LifetimesCombined<'s> {
        fn describe<'a>(&self, suffix: &'a str) -> String {
            format!("{}{}", self.prefix, suffix)
        }
    }

    #[test]
    fn test_lifetimes_combined_spy() {
        let spy = LifetimesCombined::describe_spy();
        spy.expect_once();

        let s = LifetimesCombined { prefix: "Test" };
        assert_eq!(s.describe(" Value"), "Test Value");

        spy.assert();
    }
}

mod mock {
    struct LifetimesCombined<'s> {
        prefix: &'s str,
    }

    #[fnmock::mockable]
    impl<'s> LifetimesCombined<'s> {
        fn describe<'a>(&self, suffix: &'a str) -> String {
            format!("{}{}", self.prefix, suffix)
        }
    }

    #[test]
    fn test_lifetimes_combined() {
        let mock = LifetimesCombined::describe_mock();
        mock.setup(|_, suffix| format!("Fake{}", suffix));
        mock.expect_once();

        let s = LifetimesCombined { prefix: "Test" };
        let res = s.describe(" Value");

        assert_eq!(res, "Fake Value");
        mock.assert();
    }
}
