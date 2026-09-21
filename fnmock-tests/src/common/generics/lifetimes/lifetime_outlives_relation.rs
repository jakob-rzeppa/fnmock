mod fake {
    #[fnmock::fakeable]
    fn lifetime_outlives_relation<'a, 'b: 'a>(x: &'a str, y: &'b str) -> usize {
        x.len() + y.len()
    }

    #[test]
    fn test_lifetime_outlives_relation() {
        let res = lifetime_outlives_relation("a", "bb");
        assert_eq!(res, 3);
    }

    #[test]
    fn test_lifetime_outlives_relation_fake() {
        lifetime_outlives_relation_fake().setup(|x, y| x.len() + y.len() + 1);

        let res = lifetime_outlives_relation("a", "bb");
        assert_eq!(res, 4);
    }

    #[test]
    fn test_lifetime_outlives_relation_fake_sees_a_shorter_borrow() {
        lifetime_outlives_relation_fake().setup(|x, y| x.len() + y.len() + 1);

        let long = "a".to_string();
        let res = {
            let short = "bb".to_string();
            lifetime_outlives_relation(&long, &short)
        };
        assert_eq!(res, 4);
    }
}

mod spy {
    #[fnmock::spyable]
    fn lifetime_outlives_relation<'a, 'b: 'a>(x: &'a str, y: &'b str) -> usize {
        x.len() + y.len()
    }

    #[test]
    fn test_lifetime_outlives_relation() {
        let spy = lifetime_outlives_relation_spy();
        spy.expectf(|x: &str, y: &str| x == "a" && y == "bb").once();

        let res = lifetime_outlives_relation("a", "bb");

        assert_eq!(res, 3);
        spy.assert();
    }
}
