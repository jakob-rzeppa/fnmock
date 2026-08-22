mod fake {
    #[fnmock::fakeable]
    fn raw_mut_pointers(a: *mut String) -> String {
        unsafe { (*a).clone() }
    }

    #[test]
    fn test_raw_mut_pointers() {
        let mut value = "Test".to_string();
        let result = raw_mut_pointers(&mut value as *mut String);
        assert_eq!(result, "Test");
    }

    #[test]
    fn test_raw_mut_pointers_fake() {
        let mut value = "Test".to_string();
        raw_mut_pointers_fake().setup(|a| unsafe {
            let mut clone = (*a).clone();
            clone.push_str(" fake modified");
            clone
        });
        let result = raw_mut_pointers(&mut value as *mut String);
        assert_eq!(result, "Test fake modified");
    }
}

mod spy {
    #[fnmock::spyable]
    fn raw_mut_pointers(a: *mut String) -> String {
        unsafe { (*a).clone() }
    }

    #[test]
    fn test_raw_mut_pointers() {
        let spy = raw_mut_pointers_spy();
        spy.expectf(|e| unsafe { **e == "hi".to_string() });

        let mut value = "hi".to_string();
        let result = raw_mut_pointers(&mut value as *mut String);

        assert_eq!(result, "hi");
        spy.assert();
    }
}
