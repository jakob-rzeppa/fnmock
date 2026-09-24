mod fake {
    struct BasicStruct;

    #[fnmock::fakeable]
    impl BasicStruct {
        fn basic(&self) -> i32 {
            42
        }
    }

    #[test]
    fn test_basic() {
        let s = BasicStruct;
        assert_eq!(s.basic(), 42);
    }

    #[test]
    fn test_basic_mock() {
        BasicStruct::basic_fake().setup(|_| 5);

        let s = BasicStruct;
        assert_eq!(s.basic(), 5);
    }
}

mod spy {
    struct BasicStruct;

    #[fnmock::spyable]
    impl BasicStruct {
        fn basic(&self) -> i32 {
            42
        }
    }

    #[test]
    fn test_basic_spy() {
        let spy = BasicStruct::basic_spy();
        spy.expect_once();

        let s = BasicStruct;
        assert_eq!(s.basic(), 42);

        spy.assert();
    }
}

mod mock {
    struct BasicStruct;

    #[fnmock::mockable]
    impl BasicStruct {
        fn basic(&self) -> i32 {
            42
        }
    }

    #[test]
    fn test_basic() {
        let mock = BasicStruct::basic_mock();
        mock.setup(|_| 5);
        mock.expect_once();

        let s = BasicStruct;
        let res = s.basic();

        assert_eq!(res, 5);
        mock.assert();
    }
}
