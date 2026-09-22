mod fake {
    #[fnmock::fakeable]
    fn tuple_destructuring((left, right): (String, String)) -> String {
        format!("{}{}", left, right)
    }

    #[test]
    fn test_tuple_destructuring() {
        let result = tuple_destructuring(("Test".to_string(), " Value".to_string()));
        assert_eq!(result, "Test Value");
    }

    #[test]
    fn test_tuple_destructuring_fake() {
        tuple_destructuring_fake().setup(|(left, right)| format!("Fake {}{}", left, right));

        let result = tuple_destructuring(("Test".to_string(), " Value".to_string()));
        assert_eq!(result, "Fake Test Value");
    }
}

// No `mod spy` here: spy rejects tuple-destructuring params outright, see
// unsupported/spy/tuple_destructuring.rs.
//
// No `mod mock` either, for the same reason: a mock is the intersection of
// fake and spy, see docs/LIMITATIONS.md#parameter-patterns and
// unsupported_tuple_destructuring_mock.cf.rs.
