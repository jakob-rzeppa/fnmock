mod fake {
    #[fnmock::fakeable]
    fn mutable_pattern_tuple((mut left, right): (String, String)) -> String {
        left.push_str(&right);
        left
    }

    #[test]
    fn test_mutable_pattern_tuple() {
        let result = mutable_pattern_tuple(("Test".to_string(), " Value".to_string()));
        assert_eq!(result, "Test Value");
    }

    #[test]
    fn test_mutable_pattern_tuple_fake() {
        mutable_pattern_tuple_fake().setup(|(mut left, right)| {
            left.push_str(" Fake");
            left.push_str(&right);
            left
        });

        let result = mutable_pattern_tuple(("Test".to_string(), " Value".to_string()));
        assert_eq!(result, "Test Fake Value");
    }
}

// No `mod spy` here: spy rejects tuple-destructuring params outright, even
// with a `mut` binding inside, see unsupported/spy/tuple_destructuring.rs.
