// use std::fmt::Display;

// #[fnmock::fakeable]
// fn associated_type_bounds<I>(value: I) -> Vec<String> where I: Iterator, I::Item: Display {
//     value.map(|item| item.to_string()).collect()
// }

// #[test]
// fn test_associated_type_bounds() {
//     let result = associated_type_bounds(vec!["Test".to_string()].into_iter());
//     assert_eq!(result, vec!["Test".to_string()]);
// }

// #[test]
// fn test_associated_type_bounds_fake() {
//     associated_type_bounds_fake::<std::vec::IntoIter<String>>().setup(|_value| {
//         vec!["Fake".to_string()]
//     });

//     let result = associated_type_bounds(vec!["Test".to_string()].into_iter());
//     assert_eq!(result, vec!["Fake".to_string()]);
// }
