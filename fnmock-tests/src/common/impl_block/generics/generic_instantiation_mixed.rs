mod fake {
    use std::fmt::Display;

    struct Foo<T, U> {
        value: T,
        value2: U,
    }

    #[fnmock::fakeable]
    impl<U: Display + 'static> Foo<u8, U> {
        fn bar(&self) -> u8 {
            self.value
        }

        fn bar2(self) -> U {
            self.value2
        }
    }

    #[test]
    fn test_foo_bar() {
        let foo = Foo::<u8, i32> {
            value: 1,
            value2: 2,
        };
        assert_eq!(foo.bar(), 1);
        assert_eq!(foo.bar2(), 2);
    }

    #[test]
    fn test_foo_bar_fake() {
        Foo::<u8, i32>::bar_fake().setup(|_| 6);
        Foo::<u8, i32>::bar2_fake().setup(|_| 7);
        let foo = Foo::<u8, i32> {
            value: 1,
            value2: 2,
        };
        assert_eq!(foo.bar(), 6);
        assert_eq!(foo.bar2(), 7);
    }
}

mod spy {
    use std::fmt::Display;

    struct Foo<T, U> {
        value: T,
        value2: U,
    }

    #[fnmock::spyable]
    impl<U: Display + 'static> Foo<u8, U> {
        fn bar(&self) -> u8 {
            self.value
        }

        fn bar2(self) -> U {
            self.value2
        }
    }

    #[test]
    fn test_foo_bar_spy() {
        let spy_bar = Foo::<u8, i32>::bar_spy();
        let spy_bar2 = Foo::<u8, i32>::bar2_spy();
        spy_bar.expect_once();
        spy_bar2.expect_once();

        let foo = Foo::<u8, i32> {
            value: 1,
            value2: 2,
        };
        assert_eq!(foo.bar(), 1);
        assert_eq!(foo.bar2(), 2);

        spy_bar.assert();
        spy_bar2.assert();
    }
}
