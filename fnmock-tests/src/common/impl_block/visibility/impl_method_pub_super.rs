mod fake {
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
}

mod spy {
    mod inner {
        pub struct PubSuperStruct;

        #[fnmock::spyable]
        impl PubSuperStruct {
            pub(super) fn pub_super_method(&self, a: String) -> String {
                format!("Real {}", a)
            }
        }
    }

    #[test]
    fn test_pub_super_impl_method_spy_accessor_usable_from_parent_module() {
        let spy = inner::PubSuperStruct::pub_super_method_spy();
        spy.expect_once();

        let s = inner::PubSuperStruct;
        assert_eq!(s.pub_super_method("Test".to_string()), "Real Test");

        spy.assert();
    }
}

mod mock {
    mod inner {
        pub struct PubSuperStruct;

        #[fnmock::mockable]
        impl PubSuperStruct {
            pub(super) fn pub_super_method(&self, a: String) -> String {
                format!("Real {}", a)
            }
        }
    }

    #[test]
    fn test_pub_super_impl_method_mock_accessor_usable_from_parent_module() {
        let mock = inner::PubSuperStruct::pub_super_method_mock();
        mock.setup(|_, a| format!("Fake {}", a));
        mock.expect_once();

        let s = inner::PubSuperStruct;
        let res = s.pub_super_method("Test".to_string());

        assert_eq!(res, "Fake Test");
        mock.assert();
    }
}
