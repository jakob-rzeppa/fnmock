mod fake {
    #[fnmock::fakeable]
    fn multiple_lifetimes<'a, 'b>(a: &'a str, b: &'b str) -> String {
        format!("{} {}", a, b)
    }

    #[test]
    fn test_multiple_lifetimes() {
        let value = "Test".to_string();
        let another = "Another".to_string();
        let result = multiple_lifetimes(&value, &another);
        assert_eq!(result, "Test Another");
    }

    #[test]
    fn test_multiple_lifetimes_fake() {
        let value = "Test".to_string();
        let another = "Another".to_string();
        multiple_lifetimes_fake().setup(|a, b| format!("{} {} fake modified", a, b));
        let result = multiple_lifetimes(&value, &another);
        assert_eq!(result, "Test Another fake modified");
    }
}

mod spy {
    struct Ref<'a>(&'a str);

    #[fnmock::spyable]
    fn multiple_lifetimes<'a, 'b>(left: Ref<'a>, right: Ref<'b>) -> usize {
        left.0.len() + right.0.len()
    }

    #[test]
    fn test_two_lifetimes_collapse_onto_the_matcher_lifetime() {
        let spy = multiple_lifetimes_spy();
        spy.expectf(|left: &Ref<'_>, right: &Ref<'_>| left.0 == "a" && right.0 == "bb")
            .once();

        let long = "a".to_string();
        let res = {
            let short = "bb".to_string();
            multiple_lifetimes(Ref(&long), Ref(&short))
        };

        assert_eq!(res, 3);
        spy.assert();
    }
}
