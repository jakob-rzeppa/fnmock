mod fake {
    #[fnmock::fakeable]
    fn mut_reference(a: &mut String) {
        a.push_str(" modified");
    }

    #[test]
    fn test_mut_reference() {
        let mut value = "Test".to_string();
        mut_reference(&mut value);
        assert_eq!(value, "Test modified");
    }

    #[test]
    fn test_mut_reference_fake() {
        let mut value = "Test".to_string();
        mut_reference_fake().setup(|a| a.push_str(" fake modified"));
        mut_reference(&mut value);
        assert_eq!(value, "Test fake modified");
    }
}

mod spy {
    #[fnmock::spyable]
    fn mut_reference(a: &mut String) {
        a.push_str(" modified");
    }

    #[test]
    fn test_mut_reference() {
        let spy = mut_reference_spy();
        spy.expect(fnmock::predicate::eq("hi".to_string()));

        let mut value = "hi".to_string();
        mut_reference(&mut value);

        assert_eq!(value, "hi modified");
        spy.assert();
    }
}
