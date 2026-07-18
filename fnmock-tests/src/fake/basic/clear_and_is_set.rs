#[fnmock::fakeable]
fn clear_and_is_set(a: String) -> String {
    format!("Real {}", a)
}

#[test]
fn test_is_set_transitions() {
    assert!(!clear_and_is_set_fake().is_set());

    clear_and_is_set_fake().setup(|a| format!("Fake {}", a));
    assert!(clear_and_is_set_fake().is_set());

    clear_and_is_set_fake().clear();
    assert!(!clear_and_is_set_fake().is_set());
}

#[test]
fn test_clear_restores_real_implementation() {
    clear_and_is_set_fake().setup(|a| format!("Fake {}", a));
    let res = clear_and_is_set("Test".to_string());
    assert_eq!(res, "Fake Test");

    clear_and_is_set_fake().clear();
    let res = clear_and_is_set("Test".to_string());
    assert_eq!(res, "Real Test");
}

#[test]
fn test_setup_twice_overwrites() {
    clear_and_is_set_fake().setup(|a| format!("First {}", a));
    clear_and_is_set_fake().setup(|a| format!("Second {}", a));

    let res = clear_and_is_set("Test".to_string());
    assert_eq!(res, "Second Test");
}
