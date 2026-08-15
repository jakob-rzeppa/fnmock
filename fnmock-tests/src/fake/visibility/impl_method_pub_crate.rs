mod definitions {
    pub struct CrateVisibleStruct;

    #[fnmock::fakeable]
    impl CrateVisibleStruct {
        pub(crate) fn crate_visible_method(&self, a: String) -> String {
            format!("Real {}", a)
        }
    }
}

#[test]
fn test_pub_crate_impl_method_fake_accessor_usable_from_another_module() {
    definitions::CrateVisibleStruct::crate_visible_method_fake()
        .setup(|_, a| format!("Fake {}", a));
    let s = definitions::CrateVisibleStruct;
    assert_eq!(s.crate_visible_method("Test".to_string()), "Fake Test");
}
