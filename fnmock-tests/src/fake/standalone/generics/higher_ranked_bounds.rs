// #[fnmock::fakeable]
// fn higher_ranked_bounds<F>(value: F) -> String where F: for<'a> Fn(&'a str) -> &'a str {
//     value("Real").to_string()
// }

// #[test]
// fn test_higher_ranked_bounds() {
//     let result = higher_ranked_bounds(|value| value);
//     assert_eq!(result, "Real");
// }

// #[test]
// fn test_higher_ranked_bounds_fake() {
//     higher_ranked_bounds_fake().setup(|_value| "Fake".to_string());

//     let result = higher_ranked_bounds(|value| value);
//     assert_eq!(result, "Fake");
// }
