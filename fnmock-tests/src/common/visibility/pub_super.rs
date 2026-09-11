mod fake {
    mod inner {
        #[fnmock::fakeable]
        pub(super) fn pub_super_fn(a: String) -> String {
            format!("Real {}", a)
        }
    }

    #[test]
    fn test_pub_super_fake_accessor_usable_from_parent_module() {
        inner::pub_super_fn_fake().setup(|a| format!("Fake {}", a));
        assert_eq!(inner::pub_super_fn("Test".to_string()), "Fake Test");
    }
}

mod spy {
    mod inner {
        #[fnmock::spyable]
        pub(super) fn pub_super_fn(a: String) -> String {
            format!("Real {}", a)
        }
    }

    #[test]
    fn test_pub_super_spy_accessor_usable_from_parent_module() {
        let spy = inner::pub_super_fn_spy();
        spy.expect(fnmock::predicate::eq("Test".to_string()));

        assert_eq!(inner::pub_super_fn("Test".to_string()), "Real Test");

        spy.assert();
    }
}
