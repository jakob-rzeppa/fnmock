//! `setup`/`clear` on the generated interface consume and return `Self`, so calls can be chained

#[fnmock::fakeable]
fn accessor_chaining(a: String) -> String {
    format!("Real {}", a)
}

#[test]
fn test_setup_then_clear_can_be_chained() {
    accessor_chaining_fake()
        .setup(|a| format!("Fake {}", a))
        .clear();

    let res = accessor_chaining("Test".to_string());
    assert_eq!(res, "Real Test");
}

#[test]
fn test_is_set_can_be_called_on_the_result_of_setup() {
    assert!(accessor_chaining_fake()
        .setup(|a| format!("Fake {}", a))
        .is_set());

    let res = accessor_chaining("Test".to_string());
    assert_eq!(res, "Fake Test");
}
