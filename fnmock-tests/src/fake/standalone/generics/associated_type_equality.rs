// #[fnmock::fakeable]
// fn associated_type_equality<I>(value: I) -> Vec<String> where I: Iterator<Item = String> {
//     value.collect()
// }

// #[test]
// fn test_associated_type_equality() {
//     let result = associated_type_equality(vec!["Test".to_string()].into_iter());
//     assert_eq!(result, vec!["Test".to_string()]);
// }

// #[test]
// fn test_associated_type_equality_fake() {
//     associated_type_equality_fake::<std::vec::IntoIter<String>>().setup(|_value| {
//         vec!["Fake".to_string()]
//     });

//     let result = associated_type_equality(vec!["Test".to_string()].into_iter());
//     assert_eq!(result, vec!["Fake".to_string()]);
// }
