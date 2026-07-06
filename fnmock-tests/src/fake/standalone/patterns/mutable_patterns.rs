// #[fnmock::fakeable]
// fn mutable_patterns((mut left, right): (String, String)) -> String {
//     left.push_str(&right);
//     left
// }

// #[test]
// fn test_mutable_patterns() {
//     let result = mutable_patterns(("Test".to_string(), " Value".to_string()));
//     assert_eq!(result, "Test Value");
// }

// #[test]
// fn test_mutable_patterns_fake() {
//     mutable_patterns_fake().setup(|(mut left, right)| {
//         left.push_str(" Fake");
//         left.push_str(&right);
//         left
//     });

//     let result = mutable_patterns(("Test".to_string(), " Value".to_string()));
//     assert_eq!(result, "Test Fake Value");
// }
