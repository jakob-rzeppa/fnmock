#[fnmock::fakeable]
fn nested_tuple_destructuring(((a, b), c): ((i32, i32), i32)) -> i32 {
    a + b + c
}

#[test]
fn test_nested_tuple_destructuring() {
    let result = nested_tuple_destructuring(((1, 2), 3));
    assert_eq!(result, 6);
}

#[test]
fn test_nested_tuple_destructuring_fake() {
    nested_tuple_destructuring_fake().setup(|((a, b), c)| a * b * c);

    let result = nested_tuple_destructuring(((2, 3), 4));
    assert_eq!(result, 24);
}
