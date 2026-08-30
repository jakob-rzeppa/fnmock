mod fake {
    mod definitions {
        pub mod inner {
            pub struct PubInPathStruct;

            #[fnmock::fakeable]
            impl PubInPathStruct {
                pub(in crate::common::impl_block::visibility::impl_method_pub_in_path) fn pub_in_path_method(
                    &self,
                    a: String,
                ) -> String {
                    format!("Real {}", a)
                }
            }
        }
    }

    #[test]
    fn test_pub_in_path_impl_method_fake_accessor_usable_within_declared_path() {
        definitions::inner::PubInPathStruct::pub_in_path_method_fake()
            .setup(|_, a| format!("Fake {}", a));
        let s = definitions::inner::PubInPathStruct;
        assert_eq!(s.pub_in_path_method("Test".to_string()), "Fake Test");
    }
}

mod spy {
    mod definitions {
        pub mod inner {
            pub struct PubInPathStruct;

            #[fnmock::spyable]
            impl PubInPathStruct {
                pub(in crate::common::impl_block::visibility::impl_method_pub_in_path) fn pub_in_path_method(
                    &self,
                    a: String,
                ) -> String {
                    format!("Real {}", a)
                }
            }
        }
    }

    #[test]
    fn test_pub_in_path_impl_method_spy_accessor_usable_within_declared_path() {
        let spy = definitions::inner::PubInPathStruct::pub_in_path_method_spy();
        spy.expect_once();

        let s = definitions::inner::PubInPathStruct;
        assert_eq!(s.pub_in_path_method("Test".to_string()), "Real Test");

        spy.assert();
    }
}
