mod fake {
    #[fnmock::fakeable]
    fn reference_in_tuple(pair: (&str, i32)) -> String {
        format!("Real {} {}", pair.0, pair.1)
    }

    #[test]
    fn test_reference_in_tuple() {
        let s = "Test".to_string();
        assert_eq!(reference_in_tuple((&s, 7)), "Real Test 7");
    }

    #[test]
    fn test_reference_in_tuple_fake() {
        reference_in_tuple_fake().setup(|pair| format!("Fake {} {}", pair.0, pair.1));

        let s = "Test".to_string();
        assert_eq!(reference_in_tuple((&s, 7)), "Fake Test 7");
    }
}

mod spy {
    #[fnmock::spyable]
    fn reference_in_tuple(pair: (&'static str, i32)) -> String {
        format!("Real {} {}", pair.0, pair.1)
    }

    #[test]
    fn test_reference_in_tuple_spy() {
        let spy = reference_in_tuple_spy();
        spy.expect(fnmock::predicate::eq(("hi", 42)));

        reference_in_tuple(("hi", 42));

        spy.assert();
    }
}

mod mock {
    #[fnmock::mockable]
    fn reference_in_tuple(pair: (&'static str, i32)) -> String {
        format!("Real {} {}", pair.0, pair.1)
    }

    #[test]
    fn test_reference_in_tuple() {
        let mock = reference_in_tuple_mock();
        mock.setup(|pair| format!("Fake {} {}", pair.0, pair.1));
        mock.expect(fnmock::predicate::eq(("hi", 42))).once();

        let result = reference_in_tuple(("hi", 42));

        assert_eq!(result, "Fake hi 42");
        mock.assert();
    }
}
