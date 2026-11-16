//! A reference nested in a container with no lifetime written (`Option<&str>`) is accepted by
//! every macro. A spy or mock matches it with `expectf` only: `expect`'s `Predicate<..>` has no
//! lifetime to give it.

mod spy {
    #[fnmock::spyable]
    fn elided_in_option(a: Option<&str>) -> usize {
        a.map_or(0, str::len)
    }

    #[fnmock::spyable]
    fn elided_in_vec(a: Vec<&str>) -> usize {
        a.len()
    }

    #[fnmock::spyable]
    fn elided_in_tuple(a: (&str, i32)) -> usize {
        a.0.len()
    }

    #[fnmock::spyable]
    fn elided_in_slice(a: &[&str]) -> usize {
        a.len()
    }

    #[test]
    fn test_elided_in_option_spy() {
        let spy = elided_in_option_spy();
        spy.expectf(|a: &Option<&str>| *a == Some("hi")).once();

        let value = "hi".to_string();
        assert_eq!(elided_in_option(Some(&value)), 2);

        spy.assert();
    }

    #[test]
    fn test_elided_in_vec_spy() {
        let spy = elided_in_vec_spy();
        spy.expectf(|a: &Vec<&str>| a.len() == 2).once();

        let value = "hi".to_string();
        assert_eq!(elided_in_vec(vec![&value, &value]), 2);

        spy.assert();
    }

    #[test]
    fn test_elided_in_tuple_spy() {
        let spy = elided_in_tuple_spy();
        spy.expectf(|a: &(&str, i32)| a.0 == "hi" && a.1 == 7)
            .once();

        let value = "hi".to_string();
        assert_eq!(elided_in_tuple((&value, 7)), 2);

        spy.assert();
    }

    #[test]
    fn test_elided_in_slice_spy() {
        let spy = elided_in_slice_spy();
        spy.expectf(|a: &[&str]| a == ["a", "b"]).once();

        let value = "a".to_string();
        assert_eq!(elided_in_slice(&[&value, "b"]), 2);

        spy.assert();
    }

    #[test]
    fn test_elided_in_option_spy_rejects_a_non_matching_call() {
        let spy = elided_in_option_spy();
        spy.expectf(|a: &Option<&str>| a.is_none()).once();

        let value = "hi".to_string();
        elided_in_option(Some(&value));

        let result = std::panic::catch_unwind(|| spy.assert());
        assert!(result.is_err());
    }
}

mod mock {
    #[fnmock::mockable]
    fn elided_in_option(a: Option<&str>) -> usize {
        a.map_or(0, str::len)
    }

    #[fnmock::mockable]
    fn elided_in_vec(a: Vec<&str>) -> usize {
        a.len()
    }

    #[fnmock::mockable]
    fn elided_in_tuple(a: (&str, i32)) -> usize {
        a.0.len()
    }

    #[test]
    fn test_elided_in_option_mock() {
        let mock = elided_in_option_mock();
        mock.setup(|a| a.map_or(100, str::len));
        mock.expectf(|a: &Option<&str>| a.is_none()).once();

        assert_eq!(elided_in_option(None), 100);

        mock.assert();
    }

    #[test]
    fn test_elided_in_vec_mock() {
        let mock = elided_in_vec_mock();
        mock.setup(|a| a.len() + 1);
        mock.expectf(|a: &Vec<&str>| a == &["x"]).once();

        let value = "x".to_string();
        assert_eq!(elided_in_vec(vec![&value]), 2);

        mock.assert();
    }

    #[test]
    fn test_elided_in_tuple_mock() {
        let mock = elided_in_tuple_mock();
        mock.setup(|a| a.1 as usize);
        mock.expectf(|a: &(&str, i32)| a.1 == 7).once();

        let value = "hi".to_string();
        assert_eq!(elided_in_tuple((&value, 7)), 7);

        mock.assert();
    }
}
