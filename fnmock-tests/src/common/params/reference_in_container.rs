#[fnmock::fakeable]
fn option_ref_param(a: Option<&str>) -> String {
    format!("Real {}", a.unwrap_or("none"))
}

#[test]
fn test_option_ref_param_fake() {
    option_ref_param_fake().setup(|a| format!("Fake {}", a.unwrap_or("none")));

    let value = "Test".to_string();
    assert_eq!(option_ref_param(Some(&value)), "Fake Test");
    assert_eq!(option_ref_param(None), "Fake none");
}

#[fnmock::fakeable]
fn vec_ref_param(items: Vec<&str>) -> String {
    format!("Real {}", items.join(","))
}

#[test]
fn test_vec_ref_param_fake() {
    vec_ref_param_fake().setup(|items| format!("Fake {}", items.join("|")));

    let a = "x".to_string();
    let b = "y".to_string();
    assert_eq!(vec_ref_param(vec![&a, &b]), "Fake x|y");
}

#[fnmock::fakeable]
fn slice_param(items: &[i32]) -> i32 {
    items.iter().sum()
}

#[test]
fn test_slice_param_fake() {
    slice_param_fake().setup(|items: &[i32]| items.iter().product());

    assert_eq!(slice_param(&[2, 3, 4]), 24);
}

#[fnmock::fakeable]
fn tuple_ref_param(pair: (&str, i32)) -> String {
    format!("Real {} {}", pair.0, pair.1)
}

#[test]
fn test_tuple_ref_param_fake() {
    tuple_ref_param_fake().setup(|pair| format!("Fake {} {}", pair.0, pair.1));

    let s = "Test".to_string();
    assert_eq!(tuple_ref_param((&s, 7)), "Fake Test 7");
}

#[fnmock::fakeable]
fn option_ref_return(a: &str) -> Option<&str> {
    Some(a)
}

#[test]
fn test_option_ref_return_fake() {
    option_ref_return_fake().setup(|_| None);

    let value = "Test".to_string();
    assert_eq!(option_ref_return(&value), None);
}
