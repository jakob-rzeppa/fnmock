mod definitions {
    pub mod inner {
        #[fnmock::fakeable]
        pub(in crate::fake::visibility::standalone_pub_in_path) fn pub_in_path_fn(
            a: String,
        ) -> String {
            format!("Real {}", a)
        }
    }
}

#[test]
fn test_pub_in_path_fake_accessor_usable_within_declared_path() {
    definitions::inner::pub_in_path_fn_fake().setup(|a| format!("Fake {}", a));
    assert_eq!(
        definitions::inner::pub_in_path_fn("Test".to_string()),
        "Fake Test"
    );
}
