#[fnmock::fakeable]
extern "C" fn extern_function(value: i32) -> i32 {
    value + 1
}

#[test]
fn test_extern_function() {
    let result = extern_function(1);
    assert_eq!(result, 2);
}

#[test]
fn test_extern_function_fake() {
    extern_function_fake().setup(|value| value + 10);

    let result = extern_function(1);
    assert_eq!(result, 11);
}
