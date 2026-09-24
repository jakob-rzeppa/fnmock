mod fake {
    mod definitions {
        #[fnmock::fakeable]
        pub fn publicly_visible(a: String) -> String {
            format!("Real {}", a)
        }
    }

    #[test]
    fn test_pub_fake_accessor_usable_from_another_module() {
        definitions::publicly_visible_fake().setup(|a| format!("Fake {}", a));
        assert_eq!(
            definitions::publicly_visible("Test".to_string()),
            "Fake Test"
        );
    }
}

mod spy {
    mod definitions {
        #[fnmock::spyable]
        pub fn publicly_visible(a: String) -> String {
            format!("Real {}", a)
        }
    }

    #[test]
    fn test_pub_spy_accessor_usable_from_another_module() {
        let spy = definitions::publicly_visible_spy();
        spy.expect(fnmock::predicate::eq("Test".to_string()));

        assert_eq!(
            definitions::publicly_visible("Test".to_string()),
            "Real Test"
        );
        spy.assert();
    }
}

mod mock {
    mod definitions {
        #[fnmock::mockable]
        pub fn publicly_visible(a: String) -> String {
            format!("Real {}", a)
        }
    }

    #[test]
    fn test_pub_mock_accessor_usable_from_another_module() {
        let mock = definitions::publicly_visible_mock();
        mock.setup(|a| format!("Fake {}", a));
        mock.expect(fnmock::predicate::eq("Test".to_string()))
            .once();

        let res = definitions::publicly_visible("Test".to_string());

        assert_eq!(res, "Fake Test");
        mock.assert();
    }
}
