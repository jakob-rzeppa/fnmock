mod fake {
    #[fnmock::fakeable]
    fn single_const_generic<const C: usize>(a: String) -> String {
        format!("{} {}", a, C)
    }

    #[test]
    fn test_single_const_generic() {
        let res = single_const_generic::<5>("Test".to_string());
        assert_eq!(res, "Test 5");
    }

    #[test]
    fn test_single_const_generic_fake() {
        single_const_generic_fake::<5>().setup(|a| {
            // You can't access C like this
            // format!("Fake {}", a, C)

            // So you have to hardcode it, but since you know the value of C and the fake is only for this specific value of C, it is not a problem
            format!("Fake {} {}", a, 5)
        });
        let res = single_const_generic::<5>("Test".to_string());
        assert_eq!(res, "Fake Test 5");
    }
}

mod spy {
    #[fnmock::spyable]
    fn single_const_generic<const C: usize>(a: String) -> String {
        format!("{} {}", a, C)
    }

    #[test]
    fn test_single_const_generic() {
        let spy = single_const_generic_spy::<5>();
        spy.expect(fnmock::predicate::eq("Test".to_string())).once();

        let res = single_const_generic::<5>("Test".to_string());

        assert_eq!(res, "Test 5");
        spy.assert();
    }
}
