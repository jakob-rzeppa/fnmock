//! `Self` inside a method's generic bounds, inline (`T: Into<Self>`) and in a `where` clause, has
//! to be replaced by the impl block's type just like `Self` in the parameters and return type.

mod fake {
    #[derive(Debug, PartialEq)]
    struct Meters(u32);

    impl From<u32> for Meters {
        fn from(value: u32) -> Self {
            Meters(value)
        }
    }

    #[fnmock::fakeable]
    impl Meters {
        fn convert<T: Into<Self> + 'static>(t: T) -> Self {
            t.into()
        }

        fn convert_where<T>(t: T) -> Self
        where
            T: Into<Self> + 'static,
        {
            t.into()
        }
    }

    #[derive(Debug, PartialEq)]
    struct Wrapper<W>(W);

    impl From<u32> for Wrapper<u64> {
        fn from(value: u32) -> Self {
            Wrapper(value as u64)
        }
    }

    #[fnmock::fakeable]
    impl<W: 'static> Wrapper<W> {
        fn convert<T: Into<Self> + 'static>(t: T) -> Self {
            t.into()
        }
    }

    #[test]
    fn test_self_in_method_bounds() {
        assert_eq!(Meters::convert(5u32), Meters(5));
        assert_eq!(Meters::convert_where(5u32), Meters(5));
        assert_eq!(Wrapper::<u64>::convert(5u32), Wrapper(5));
    }

    #[test]
    fn test_self_in_method_bounds_fake() {
        Meters::convert_fake::<u32>().setup(|t| Meters(t * 2));
        Meters::convert_where_fake::<u32>().setup(|t| Meters(t * 3));
        Wrapper::<u64>::convert_fake::<u32>().setup(|t| Wrapper(t as u64 * 4));

        assert_eq!(Meters::convert(5u32), Meters(10));
        assert_eq!(Meters::convert_where(5u32), Meters(15));
        assert_eq!(Wrapper::<u64>::convert(5u32), Wrapper(20));
    }
}

mod spy {
    #[derive(Debug, PartialEq)]
    struct Meters(u32);

    impl From<u32> for Meters {
        fn from(value: u32) -> Self {
            Meters(value)
        }
    }

    #[fnmock::spyable]
    impl Meters {
        fn convert<T: Into<Self> + 'static>(t: T) -> Self {
            t.into()
        }

        fn convert_where<T>(t: T) -> Self
        where
            T: Into<Self> + 'static,
        {
            t.into()
        }
    }

    #[derive(Debug, PartialEq)]
    struct Wrapper<W>(W);

    impl From<u32> for Wrapper<u64> {
        fn from(value: u32) -> Self {
            Wrapper(value as u64)
        }
    }

    #[fnmock::spyable]
    impl<W: 'static> Wrapper<W> {
        fn convert<T: Into<Self> + 'static>(t: T) -> Self {
            t.into()
        }
    }

    #[test]
    fn test_self_in_method_bounds_spy() {
        let convert_spy = Meters::convert_spy::<u32>();
        let convert_where_spy = Meters::convert_where_spy::<u32>();
        let wrapper_spy = Wrapper::<u64>::convert_spy::<u32>();
        convert_spy.expect(fnmock::predicate::eq(5u32)).once();
        convert_where_spy.expect(fnmock::predicate::eq(6u32)).once();
        wrapper_spy.expect(fnmock::predicate::eq(7u32)).once();

        assert_eq!(Meters::convert(5u32), Meters(5));
        assert_eq!(Meters::convert_where(6u32), Meters(6));
        assert_eq!(Wrapper::<u64>::convert(7u32), Wrapper(7));

        convert_spy.assert();
        convert_where_spy.assert();
        wrapper_spy.assert();
    }
}

mod mock {
    #[derive(Debug, PartialEq)]
    struct Meters(u32);

    impl From<u32> for Meters {
        fn from(value: u32) -> Self {
            Meters(value)
        }
    }

    #[fnmock::mockable]
    impl Meters {
        fn convert<T: Into<Self> + 'static>(t: T) -> Self {
            t.into()
        }

        fn convert_where<T>(t: T) -> Self
        where
            T: Into<Self> + 'static,
        {
            t.into()
        }
    }

    #[derive(Debug, PartialEq)]
    struct Wrapper<W>(W);

    impl From<u32> for Wrapper<u64> {
        fn from(value: u32) -> Self {
            Wrapper(value as u64)
        }
    }

    #[fnmock::mockable]
    impl<W: 'static> Wrapper<W> {
        fn convert<T: Into<Self> + 'static>(t: T) -> Self {
            t.into()
        }
    }

    #[test]
    fn test_self_in_method_bounds_mock() {
        let convert_mock = Meters::convert_mock::<u32>();
        convert_mock.setup(|t| Meters(t * 2));
        convert_mock.expect(fnmock::predicate::eq(5u32)).once();

        let convert_where_mock = Meters::convert_where_mock::<u32>();
        convert_where_mock
            .expect(fnmock::predicate::eq(6u32))
            .once();

        let wrapper_mock = Wrapper::<u64>::convert_mock::<u32>();
        wrapper_mock.setup(|t| Wrapper(t as u64 * 4));
        wrapper_mock.expect(fnmock::predicate::eq(7u32)).once();

        assert_eq!(Meters::convert(5u32), Meters(10));
        assert_eq!(Meters::convert_where(6u32), Meters(6));
        assert_eq!(Wrapper::<u64>::convert(7u32), Wrapper(28));

        convert_mock.assert();
        convert_where_mock.assert();
        wrapper_mock.assert();
    }
}
