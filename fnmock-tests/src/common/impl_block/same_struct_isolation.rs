mod fake {
    //! Two `#[fnmock::fakeable]` impl blocks for the same struct
    //! should not interfere with each other.

    struct Foo {
        value: i32,
    }

    #[fnmock::fakeable]
    impl Foo {
        fn bar_one(&self) -> i32 {
            self.value
        }
    }

    #[fnmock::fakeable]
    impl Foo {
        fn bar_two(&self) -> i32 {
            self.value + 2
        }
    }

    #[test]
    fn test_real_bodies_are_independent() {
        let first = Foo { value: 1 };
        assert_eq!(first.bar_one(), 1);

        let second = Foo { value: 2 };
        assert_eq!(second.bar_two(), 4);
    }

    #[test]
    fn test_fakes_are_independent() {
        Foo::bar_one_fake().setup(|_| 9);
        assert_eq!(Foo { value: 1 }.bar_one(), 9);
        assert_eq!(Foo { value: 2 }.bar_two(), 4);

        Foo::bar_two_fake().setup(|_| 99);
        assert_eq!(Foo { value: 1 }.bar_one(), 9);
        assert_eq!(Foo { value: 2 }.bar_two(), 99);
    }
}

mod spy {
    //! Two `#[fnmock::spyable]` impl blocks for the same struct
    //! should not interfere with each other.

    struct Foo {
        value: i32,
    }

    #[fnmock::spyable]
    impl Foo {
        fn bar_one(&self) -> i32 {
            self.value
        }
    }

    #[fnmock::spyable]
    impl Foo {
        fn bar_two(&self) -> i32 {
            self.value + 2
        }
    }

    #[test]
    fn test_spies_are_independent() {
        let spy_one = Foo::bar_one_spy();
        let spy_two = Foo::bar_two_spy();
        spy_one.expect_once();
        spy_two.expect_once();

        let first = Foo { value: 1 };
        assert_eq!(first.bar_one(), 1);

        let second = Foo { value: 2 };
        assert_eq!(second.bar_two(), 4);

        spy_one.assert();
        spy_two.assert();
    }
}
