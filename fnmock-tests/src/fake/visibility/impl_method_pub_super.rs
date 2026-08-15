mod inner {
    pub struct PubSuperStruct;

    #[fnmock::fakeable]
    impl PubSuperStruct {
        pub(super) fn pub_super_method(&self, a: String) -> String {
            format!("Real {}", a)
        }
    }
}

#[test]
fn test_pub_super_impl_method_fake_accessor_usable_from_parent_module() {
    inner::PubSuperStruct::pub_super_method_fake().setup(|_, a| format!("Fake {}", a));
    let s = inner::PubSuperStruct;
    assert_eq!(s.pub_super_method("Test".to_string()), "Fake Test");
}
