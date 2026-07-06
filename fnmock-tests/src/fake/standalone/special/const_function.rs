// #[fnmock::fakeable]
// const fn const_function(value: i32) -> i32 {
//     value + 1
// }

// #[test]
// fn test_const_function() {
//     let result = const_function(1);
//     assert_eq!(result, 2);
// }

// #[test]
// fn test_const_function_fake() {
//     const_function_fake().setup(|value| value + 10);

//     let result = const_function(1);
//     assert_eq!(result, 11);
// }
