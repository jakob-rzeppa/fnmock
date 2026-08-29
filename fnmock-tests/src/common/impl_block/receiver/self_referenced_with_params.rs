mod fake {
    struct SelfReferencedWithParams {
        base: i32,
    }

    #[fnmock::fakeable]
    impl SelfReferencedWithParams {
        fn add(&self, a: i32, b: i32) -> i32 {
            self.base + a + b
        }
    }

    #[test]
    fn test_self_referenced_with_params() {
        let s = SelfReferencedWithParams { base: 10 };
        assert_eq!(s.add(1, 2), 13);
    }

    #[test]
    fn test_self_referenced_with_params_fake() {
        SelfReferencedWithParams::add_fake().setup(|_, a, b| a * b);

        let s = SelfReferencedWithParams { base: 10 };
        assert_eq!(s.add(3, 4), 12);
    }
}

mod spy {
    struct SelfReferencedWithParams {
        base: i32,
    }

    #[fnmock::spyable]
    impl SelfReferencedWithParams {
        fn add(&self, a: i32, b: i32) -> i32 {
            self.base + a + b
        }
    }

    #[test]
    fn test_self_referenced_with_params_spy() {
        let spy = SelfReferencedWithParams::add_spy();
        spy.expect_once();

        let s = SelfReferencedWithParams { base: 10 };
        assert_eq!(s.add(1, 2), 13);

        spy.assert();
    }
}
