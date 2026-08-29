mod fake {
    //! Two `#[fnmock::fakeable]` impl blocks for the same generic struct, instantiated with different
    //! concrete type arguments (`Foo<u8>` vs `Foo<u16>`), must have independent fakes and real bodies.
    //! This is distinct from `impl_block/generics/generic_struct.rs` (a single `impl<T> Foo<T>` shared
    //! across instantiations) — here there are two separate, non-generic impl blocks.

    struct Foo<T> {
        value: T,
    }

    #[fnmock::fakeable]
    impl Foo<u8> {
        fn bar(&self) -> u8 {
            self.value
        }
    }

    #[fnmock::fakeable]
    impl Foo<u16> {
        fn bar(&self) -> u16 {
            self.value
        }
    }

    #[test]
    fn test_real_bodies_are_independent() {
        let f8 = Foo::<u8> { value: 1 };
        assert_eq!(f8.bar(), 1);

        let f16 = Foo::<u16> { value: 2 };
        assert_eq!(f16.bar(), 2);
    }

    #[test]
    fn test_fakes_are_independent() {
        Foo::<u8>::bar_fake().setup(|_| 9);
        assert_eq!(Foo::<u8> { value: 1 }.bar(), 9);
        assert_eq!(Foo::<u16> { value: 2 }.bar(), 2);

        Foo::<u16>::bar_fake().setup(|_| 99);
        assert_eq!(Foo::<u8> { value: 1 }.bar(), 9);
        assert_eq!(Foo::<u16> { value: 2 }.bar(), 99);
    }
}

mod spy {
    //! Two `#[fnmock::spyable]` impl blocks for the same generic struct, instantiated with different
    //! concrete type arguments (`Foo<u8>` vs `Foo<u16>`), must have independent spies.

    struct Foo<T> {
        value: T,
    }

    #[fnmock::spyable]
    impl Foo<u8> {
        fn bar(&self) -> u8 {
            self.value
        }
    }

    #[fnmock::spyable]
    impl Foo<u16> {
        fn bar(&self) -> u16 {
            self.value
        }
    }

    #[test]
    fn test_spies_are_independent() {
        let spy8 = Foo::<u8>::bar_spy();
        let spy16 = Foo::<u16>::bar_spy();
        spy8.expect_once();
        spy16.expect_once();

        let f8 = Foo::<u8> { value: 1 };
        assert_eq!(f8.bar(), 1);

        let f16 = Foo::<u16> { value: 2 };
        assert_eq!(f16.bar(), 2);

        spy8.assert();
        spy16.assert();
    }
}
