//! `clear()` / `is_set()` on a non-generic impl-block method. Access is a method
//! (`Struct::method_fake()`) rather than a free `_fake()` function, though the underlying
//! store is the same non-generic `FakeStore` as for free functions.

struct ClearAndIsSet;

#[fnmock::fakeable]
impl ClearAndIsSet {
    fn greet(&self, a: String) -> String {
        format!("Real {}", a)
    }
}

#[test]
fn test_is_set_transitions() {
    assert!(!ClearAndIsSet::greet_fake().is_set());

    ClearAndIsSet::greet_fake().setup(|_, a| format!("Fake {}", a));
    assert!(ClearAndIsSet::greet_fake().is_set());

    ClearAndIsSet::greet_fake().clear();
    assert!(!ClearAndIsSet::greet_fake().is_set());
}

#[test]
fn test_clear_restores_real_implementation() {
    let s = ClearAndIsSet;

    ClearAndIsSet::greet_fake().setup(|_, a| format!("Fake {}", a));
    assert_eq!(s.greet("Test".to_string()), "Fake Test");

    ClearAndIsSet::greet_fake().clear();
    assert_eq!(s.greet("Test".to_string()), "Real Test");
}

#[test]
fn test_setup_twice_overwrites() {
    let s = ClearAndIsSet;

    ClearAndIsSet::greet_fake().setup(|_, a| format!("First {}", a));
    ClearAndIsSet::greet_fake().setup(|_, a| format!("Second {}", a));

    assert_eq!(s.greet("Test".to_string()), "Second Test");
}
