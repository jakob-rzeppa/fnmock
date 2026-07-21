#[fnmock::fakeable]
fn zero_args() -> i32 {
    1
}

#[test]
fn test_zero_args() {
    let res = zero_args();
    assert_eq!(res, 1);
}

#[test]
fn test_zero_args_fake() {
    zero_args_fake().setup(|| 42);
    let res = zero_args();
    assert_eq!(res, 42);
}
