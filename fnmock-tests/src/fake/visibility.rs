//! The generated `_fake()` accessor is emitted as `#[cfg(test)] pub(crate)`, so it must be
//! usable from a different module than the one that defines the fakeable item. Every other
//! test defines and uses the fake in the same module; these exercise cross-module access
//! for both a `pub(crate)` and a `pub` fakeable function.

mod definitions {
    #[fnmock::fakeable]
    pub(crate) fn crate_visible(a: String) -> String {
        format!("Real {}", a)
    }

    #[fnmock::fakeable]
    pub fn publicly_visible(a: String) -> String {
        format!("Real {}", a)
    }
}

#[test]
fn test_pub_crate_fake_accessor_usable_from_another_module() {
    definitions::crate_visible_fake().setup(|a| format!("Fake {}", a));
    assert_eq!(definitions::crate_visible("Test".to_string()), "Fake Test");
}

#[test]
fn test_pub_fake_accessor_usable_from_another_module() {
    definitions::publicly_visible_fake().setup(|a| format!("Fake {}", a));
    assert_eq!(
        definitions::publicly_visible("Test".to_string()),
        "Fake Test"
    );
}
