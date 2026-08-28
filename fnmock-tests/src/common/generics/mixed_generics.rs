mod fake {
    #[fnmock::fakeable]
    fn mixed_generics<'a, T: std::fmt::Display + 'static, const N: usize>(a: &'a T) -> String {
        format!("{} {}", a, N)
    }

    #[test]
    fn test_mixed_generics() {
        let value = "Test".to_string();
        let res = mixed_generics::<String, 5>(&value);
        assert_eq!(res, "Test 5");
    }

    #[test]
    fn test_mixed_generics_fake() {
        mixed_generics_fake::<String, 5>().setup(|a| format!("Fake {} 5", a));

        let value = "Test".to_string();
        let res = mixed_generics::<String, 5>(&value);
        assert_eq!(res, "Fake Test 5");
    }
}

mod spy {
    #[fnmock::spyable]
    fn mixed_generics<'a, T: std::fmt::Display + 'static, const N: usize>(a: &'a T) -> String {
        format!("{} {}", a, N)
    }

    #[test]
    fn test_mixed_generics() {
        let spy = mixed_generics_spy::<String, 5>();
        spy.expect(fnmock::predicate::eq("Test".to_string())).once();

        let value = "Test".to_string();
        let res = mixed_generics::<String, 5>(&value);

        assert_eq!(res, "Test 5");
        spy.assert();
    }
}
