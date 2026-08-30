mod fake {
    #[fnmock::fakeable]
    fn nested_lifetime_in_container<'a>(items: &'a [&'a str]) -> usize {
        items.len()
    }

    #[test]
    fn test_nested_lifetime_in_container() {
        let res = nested_lifetime_in_container(&["a", "b"]);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_nested_lifetime_in_container_fake() {
        nested_lifetime_in_container_fake().setup(|items| items.len() + 1);
        let res = nested_lifetime_in_container(&["a", "b"]);
        assert_eq!(res, 3);
    }
}

mod spy {
    #[fnmock::spyable]
    fn nested_lifetime_in_container<'a>(items: &'a [&'a str]) -> usize {
        items.len()
    }

    #[test]
    fn test_nested_lifetime_in_container() {
        let spy = nested_lifetime_in_container_spy();
        spy.expectf(|items: &[&str]| items == ["a", "b"]).once();

        let owned = ["a".to_string(), "b".to_string()];
        let borrowed: Vec<&str> = owned.iter().map(String::as_str).collect();
        let res = nested_lifetime_in_container(&borrowed);

        assert_eq!(res, 2);
        spy.assert();
    }
}
