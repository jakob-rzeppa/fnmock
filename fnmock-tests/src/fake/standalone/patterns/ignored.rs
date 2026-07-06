// #[fnmock::fakeable]
// fn ignored(_: String, value: String) -> String {
//     value
// }

// #[test]
// fn test_ignored() {
//     let result = ignored("Ignored".to_string(), "Test".to_string());
//     assert_eq!(result, "Test");
// }

// #[test]
// fn test_ignored_fake() {
//     ignored_fake().setup(|_, value| format!("Fake {}", value));

//     let result = ignored("Ignored".to_string(), "Test".to_string());
//     assert_eq!(result, "Fake Test");
// }
