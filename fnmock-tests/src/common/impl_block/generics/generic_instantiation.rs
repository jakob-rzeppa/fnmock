mod fake {
    struct Foo<T> {
        value: T,
    }

    #[fnmock::fakeable]
    impl Foo<u8> {
        fn bar(&self) -> u8 {
            self.value
        }
    }

    #[test]
    fn test_foo_bar() {
        let f8 = Foo::<u8> { value: 1 };
        assert_eq!(f8.bar(), 1);
    }

    #[test]
    fn test_foo_bar_fake() {
        Foo::<u8>::bar_fake().setup(|_| 9);
        assert_eq!(Foo::<u8> { value: 1 }.bar(), 9);
    }
}

mod spy {
    struct Foo<T> {
        value: T,
    }

    #[fnmock::spyable]
    impl Foo<u8> {
        fn bar(&self) -> u8 {
            self.value
        }
    }

    #[test]
    fn test_foo_bar_spy() {
        let spy = Foo::<u8>::bar_spy();
        spy.expect_once();

        let f8 = Foo::<u8> { value: 1 };
        assert_eq!(f8.bar(), 1);

        spy.assert();
    }
}
