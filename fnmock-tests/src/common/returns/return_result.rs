mod fake {
    #[fnmock::fakeable]
    fn return_result(a: String) -> Result<String, String> {
        Ok(a)
    }

    #[test]
    fn test_return_result() {
        let res = return_result("Test".to_string());
        assert_eq!(res, Ok("Test".to_string()));
    }

    #[test]
    fn test_return_result_fake() {
        return_result_fake().setup(|a| Ok(format!("Fake {}", a)));
        let res = return_result("Test".to_string());
        assert_eq!(res, Ok("Fake Test".to_string()));
    }
}

mod spy {
    #[fnmock::spyable]
    fn return_result(a: String) -> Result<String, String> {
        Ok(a)
    }

    #[test]
    fn test_return_result_spy() {
        let spy = return_result_spy();
        spy.expect(fnmock::predicate::eq("hi".to_string()));

        assert!(return_result("hi".to_string()).is_ok());

        spy.assert();
    }
}
