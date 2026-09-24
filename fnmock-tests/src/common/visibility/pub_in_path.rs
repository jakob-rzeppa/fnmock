mod fake {
    mod definitions {
        pub mod inner {
            #[fnmock::fakeable]
            pub(in crate::common::visibility::pub_in_path::fake) fn pub_in_path_fn(
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
            pub(in crate::common::visibility::pub_in_path::spy) fn pub_in_path_fn(
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

mod mock {
    mod definitions {
        pub mod inner {
            #[fnmock::mockable]
            pub(in crate::common::visibility::pub_in_path::mock) fn pub_in_path_fn(
                a: String,
            ) -> String {
                format!("Real {}", a)
            }
        }
    }

    #[test]
    fn test_pub_in_path_mock_accessor_usable_within_declared_path() {
        let mock = definitions::inner::pub_in_path_fn_mock();
        mock.setup(|a| format!("Fake {}", a));
        mock.expect(fnmock::predicate::eq("Test".to_string()))
            .once();

        let res = definitions::inner::pub_in_path_fn("Test".to_string());

        assert_eq!(res, "Fake Test");
        mock.assert();
    }
}
