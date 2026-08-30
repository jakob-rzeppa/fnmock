mod fake {
    #[fnmock::fakeable]
    fn associated_type_equality<I: Iterator<Item = String> + 'static>(value: I) -> usize {
        value.count()
    }

    #[test]
    fn test_associated_type_equality() {
        let result = associated_type_equality(vec!["a".to_string(), "b".to_string()].into_iter());
        assert_eq!(result, 2);
    }

    #[test]
    fn test_associated_type_equality_fake() {
        associated_type_equality_fake::<std::vec::IntoIter<String>>().setup(|_value| 99);

        let result = associated_type_equality(vec!["a".to_string(), "b".to_string()].into_iter());
        assert_eq!(result, 99);
    }
}

mod spy {
    #[fnmock::spyable]
    fn associated_type_equality<I: Iterator<Item = String> + 'static>(value: I) -> usize {
        value.count()
    }

    #[test]
    fn test_associated_type_equality() {
        let spy = associated_type_equality_spy::<std::vec::IntoIter<String>>();
        spy.expectf(|value: &std::vec::IntoIter<String>| value.len() == 2)
            .once();

        let res = associated_type_equality(vec!["a".to_string(), "b".to_string()].into_iter());

        assert_eq!(res, 2);
        spy.assert();
    }
}
