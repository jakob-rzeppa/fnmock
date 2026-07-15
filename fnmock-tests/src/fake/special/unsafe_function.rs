#[fnmock::fakeable]
unsafe fn unsafe_function(value: i32) -> i32 {
    value + 1
}

#[test]
fn test_unsafe_function() {
    let result = unsafe { unsafe_function(1) };
    assert_eq!(result, 2);
}

#[test]
fn test_unsafe_function_fake() {
    unsafe_function_fake().setup(|value| value + 10);

    let result = unsafe { unsafe_function(1) };
    assert_eq!(result, 11);
}
