mod fake {
    mod definitions {
        #[fnmock::fakeable]
        pub(crate) fn crate_visible(a: String) -> String {
            format!("Real {}", a)
        }
    }

    #[test]
    fn test_pub_crate_fake_accessor_usable_from_another_module() {
        definitions::crate_visible_fake().setup(|a| format!("Fake {}", a));
        assert_eq!(definitions::crate_visible("Test".to_string()), "Fake Test");
    }
}

mod spy {
    mod definitions {
        #[fnmock::spyable]
        pub(crate) fn crate_visible(a: String) -> String {
            format!("Real {}", a)
        }
    }

    #[test]
    fn test_pub_crate_spy_accessor_usable_from_another_module() {
        let spy = definitions::crate_visible_spy();
        spy.expect(fnmock::predicate::eq("Test".to_string()));

        assert_eq!(definitions::crate_visible("Test".to_string()), "Real Test");
        spy.assert();
    }
}
