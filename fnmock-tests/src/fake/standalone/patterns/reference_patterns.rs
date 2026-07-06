// #[fnmock::fakeable]
// fn reference_patterns((&left, &right): (&i32, &i32)) -> i32 {
//     left + right
// }

// #[test]
// fn test_reference_patterns() {
//     let left = 10;
//     let right = 20;
//     let result = reference_patterns((&left, &right));
//     assert_eq!(result, 30);
// }

// #[test]
// fn test_reference_patterns_fake() {
//     reference_patterns_fake().setup(|(_, _)| 99);

//     let left = 10;
//     let right = 20;
//     let result = reference_patterns((&left, &right));
//     assert_eq!(result, 99);
// }
