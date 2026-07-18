//! `clear()` / `is_set()` on a const-generic fake. Const generic parameters are keyed by
//! value (`ConstValue`) rather than `TypeId`, a separate key path in `GenericFakeStore`.
//! Mirror of the value-isolation test in `const_generics.rs`, but for clear/is_set.

#[fnmock::fakeable]
fn const_clear_and_is_set<const C: usize>(a: String) -> String {
    format!("Real {} {}", a, C)
}

#[test]
fn test_is_set_transitions() {
    assert!(!const_clear_and_is_set_fake::<5>().is_set());

    const_clear_and_is_set_fake::<5>().setup(|a| format!("Fake {} 5", a));
    assert!(const_clear_and_is_set_fake::<5>().is_set());

    const_clear_and_is_set_fake::<5>().clear();
    assert!(!const_clear_and_is_set_fake::<5>().is_set());
}

#[test]
fn test_clear_restores_real_implementation() {
    const_clear_and_is_set_fake::<5>().setup(|a| format!("Fake {} 5", a));
    assert_eq!(const_clear_and_is_set::<5>("Test".to_string()), "Fake Test 5");

    const_clear_and_is_set_fake::<5>().clear();
    assert_eq!(const_clear_and_is_set::<5>("Test".to_string()), "Real Test 5");
}

#[test]
fn test_clear_one_value_leaves_other_values_installed() {
    const_clear_and_is_set_fake::<5>().setup(|a| format!("Fake {} 5", a));
    const_clear_and_is_set_fake::<7>().setup(|a| format!("Fake {} 7", a));

    assert!(const_clear_and_is_set_fake::<5>().is_set());
    assert!(const_clear_and_is_set_fake::<7>().is_set());

    // Clearing C=5 must leave C=7 untouched: const parameters are keyed by value.
    const_clear_and_is_set_fake::<5>().clear();

    assert!(!const_clear_and_is_set_fake::<5>().is_set());
    assert!(const_clear_and_is_set_fake::<7>().is_set());

    assert_eq!(const_clear_and_is_set::<5>("Test".to_string()), "Real Test 5");
    assert_eq!(const_clear_and_is_set::<7>("Test".to_string()), "Fake Test 7");
}
