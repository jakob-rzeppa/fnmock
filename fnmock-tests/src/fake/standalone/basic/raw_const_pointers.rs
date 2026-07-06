#[fnmock::fakeable]
fn raw_const_pointers(a: *const String) -> String {
    unsafe { (*a).clone() }
}

#[test]
fn test_raw_const_pointers() {
    let value = "Test".to_string();
    let result = raw_const_pointers(&value as *const String);
    assert_eq!(result, "Test");
}

#[test]
fn test_raw_const_pointers_fake() {
    let value = "Test".to_string();
    raw_const_pointers_fake().setup(|a| {
        unsafe {
            let mut clone = (*a).clone();
            clone.push_str(" fake modified");
            clone
        }
    });
    let result = raw_const_pointers(&value as *const String);
    assert_eq!(result, "Test fake modified");
}
