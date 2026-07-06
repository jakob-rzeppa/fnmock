// struct Point {
//     x: i32,
//     y: i32,
// }

// #[fnmock::fakeable]
// fn struct_destructuring(Point { x, y }: Point) -> i32 {
//     x + y
// }

// #[test]
// fn test_struct_destructuring() {
//     let result = struct_destructuring(Point { x: 10, y: 20 });
//     assert_eq!(result, 30);
// }

// #[test]
// fn test_struct_destructuring_fake() {
//     struct_destructuring_fake().setup(|Point { x, y }| x + y + 1);

//     let result = struct_destructuring(Point { x: 10, y: 20 });
//     assert_eq!(result, 31);
// }
