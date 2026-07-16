#[fnmock::fakeable]
fn non_parameter_where<T>(a: T) -> Vec<T> where T: 'static, Vec<T>: Clone {
    vec![a].clone()
}

#[test]
fn test_non_parameter_where() {
    let res = non_parameter_where("Test".to_string());
    assert_eq!(res, vec!["Test".to_string()]);
}

#[test]
fn test_non_parameter_where_fake() {
    non_parameter_where_fake::<String>().setup(|a| vec![format!("Fake {}", a)]);
    let res = non_parameter_where("Test".to_string());
    assert_eq!(res, vec!["Fake Test".to_string()]);
}
