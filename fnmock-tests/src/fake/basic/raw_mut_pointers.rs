#[fnmock::fakeable]
fn raw_mut_pointers(a: *mut String) {
    unsafe {
        (*a).push_str(" modified");
    }
}

#[test]
fn test_raw_mut_pointers() {
    let mut value = "Test".to_string();
    raw_mut_pointers(&mut value as *mut String);
    assert_eq!(value, "Test modified");
}

#[test]
fn test_raw_mut_pointers_fake() {
    let mut value = "Test".to_string();
    raw_mut_pointers_fake().setup(|a| {
        unsafe {
            (*a).push_str(" fake modified");
        }
    });
    raw_mut_pointers(&mut value as *mut String);
    assert_eq!(value, "Test fake modified");
}
