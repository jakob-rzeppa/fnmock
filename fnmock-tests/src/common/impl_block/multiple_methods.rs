mod fake {
    struct MultipleMethods;

    #[fnmock::fakeable]
    impl MultipleMethods {
        fn first(&self) -> i32 {
            1
        }

        fn second(&self) -> i32 {
            2
        }
    }

    #[test]
    fn test_multiple_methods() {
        let s = MultipleMethods;
        assert_eq!(s.first(), 1);
        assert_eq!(s.second(), 2);
    }

    #[test]
    fn test_multiple_methods_fake() {
        MultipleMethods::first_fake().setup(|_| 10);
        MultipleMethods::second_fake().setup(|_| 20);

        let s = MultipleMethods;
        assert_eq!(s.first(), 10);
        assert_eq!(s.second(), 20);
    }
}

mod spy {
    struct MultipleMethods;

    #[fnmock::spyable]
    impl MultipleMethods {
        fn first(&self) -> i32 {
            1
        }

        fn second(&self) -> i32 {
            2
        }
    }

    #[test]
    fn test_multiple_methods_spy() {
        let spy_first = MultipleMethods::first_spy();
        let spy_second = MultipleMethods::second_spy();
        spy_first.expect_once();
        spy_second.expect_once();

        let s = MultipleMethods;
        assert_eq!(s.first(), 1);
        assert_eq!(s.second(), 2);

        spy_first.assert();
        spy_second.assert();
    }
}
