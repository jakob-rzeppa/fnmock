mod fake {
    mod definitions {
        pub mod inner {
            #[fnmock::fakeable]
            pub(in crate::common::visibility::standalone_pub_in_path::fake) fn pub_in_path_fn(
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
}

mod spy {
    mod definitions {
        pub mod inner {
            #[fnmock::spyable]
            pub(in crate::common::visibility::standalone_pub_in_path::spy) fn pub_in_path_fn(
                a: String,
            ) -> String {
                format!("Real {}", a)
            }
        }
    }

    #[test]
    fn test_pub_in_path_spy_accessor_usable_within_declared_path() {
        let spy = definitions::inner::pub_in_path_fn_spy();
        spy.expect(fnmock::predicate::eq("Test".to_string()));

        assert_eq!(
            definitions::inner::pub_in_path_fn("Test".to_string()),
            "Real Test"
        );

        spy.assert();
    }
}
