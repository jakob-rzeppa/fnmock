mod definitions {
    pub struct PubStruct;

    #[fnmock::fakeable]
    impl PubStruct {
        pub fn pub_method(&self, a: String) -> String {
            format!("Real {}", a)
        }
    }
}

#[test]
fn test_pub_impl_method_fake_accessor_usable_from_another_module() {
    definitions::PubStruct::pub_method_fake().setup(|_, a| format!("Fake {}", a));
    let s = definitions::PubStruct;
    assert_eq!(s.pub_method("Test".to_string()), "Fake Test");
}
