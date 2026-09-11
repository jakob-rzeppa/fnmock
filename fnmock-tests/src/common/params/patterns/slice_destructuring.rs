mod fake {
    #[fnmock::fakeable]
    fn slice_destructuring([left, right]: [String; 2]) -> String {
        format!("{}{}", left, right)
    }

    #[test]
    fn test_slice_destructuring() {
        let result = slice_destructuring(["Test".to_string(), " Value".to_string()]);
        assert_eq!(result, "Test Value");
    }

    #[test]
    fn test_slice_destructuring_fake() {
        slice_destructuring_fake().setup(|[left, right]| format!("Fake {}{}", left, right));

        let result = slice_destructuring(["Test".to_string(), " Value".to_string()]);
        assert_eq!(result, "Fake Test Value");
    }
}

// No `mod spy` here: spy rejects slice-destructuring params outright, see
// unsupported/spy/slice_destructuring.rs.
