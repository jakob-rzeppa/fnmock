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
