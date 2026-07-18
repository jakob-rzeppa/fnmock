//! `clear()` / `is_set()` on a generic fake go through `GenericFakeStore::clear_for` /
//! `is_set_for` — a different code path from the non-generic `FakeStore` used by
//! `basic/clear_and_is_set.rs`. These mirror the non-generic contracts for the generic path
//! and pin that clearing one type instantiation leaves other instantiations installed.

#[fnmock::fakeable]
fn generic_clear_and_is_set<T: 'static + std::fmt::Display>(a: T) -> String {
    format!("Real {}", a)
}

#[test]
fn test_is_set_transitions() {
    assert!(!generic_clear_and_is_set_fake::<String>().is_set());

    generic_clear_and_is_set_fake::<String>().setup(|a| format!("Fake {}", a));
    assert!(generic_clear_and_is_set_fake::<String>().is_set());

    generic_clear_and_is_set_fake::<String>().clear();
    assert!(!generic_clear_and_is_set_fake::<String>().is_set());
}

#[test]
fn test_clear_restores_real_implementation() {
    generic_clear_and_is_set_fake::<String>().setup(|a| format!("Fake {}", a));
    assert_eq!(generic_clear_and_is_set("Test".to_string()), "Fake Test");

    generic_clear_and_is_set_fake::<String>().clear();
    assert_eq!(generic_clear_and_is_set("Test".to_string()), "Real Test");
}

#[test]
fn test_setup_twice_overwrites() {
    generic_clear_and_is_set_fake::<String>().setup(|a| format!("First {}", a));
    generic_clear_and_is_set_fake::<String>().setup(|a| format!("Second {}", a));

    assert_eq!(generic_clear_and_is_set("Test".to_string()), "Second Test");
}

#[test]
fn test_clear_one_instantiation_leaves_others_installed() {
    generic_clear_and_is_set_fake::<String>().setup(|a| format!("Fake {}", a));
    generic_clear_and_is_set_fake::<i32>().setup(|a| format!("Faked {}", a));

    assert!(generic_clear_and_is_set_fake::<String>().is_set());
    assert!(generic_clear_and_is_set_fake::<i32>().is_set());

    generic_clear_and_is_set_fake::<String>().clear();

    // Clearing the String instantiation must leave the i32 one untouched.
    assert!(!generic_clear_and_is_set_fake::<String>().is_set());
    assert!(generic_clear_and_is_set_fake::<i32>().is_set());

    assert_eq!(generic_clear_and_is_set("Test".to_string()), "Real Test");
    assert_eq!(generic_clear_and_is_set(42), "Faked 42");
}
