mod fake {
    #[fnmock::fakeable]
    fn non_parameter_where_with_lifetime<'a, T: 'static + Clone>(items: &'a [T]) -> usize
    where
        Vec<&'a T>: Clone,
    {
        items.len()
    }

    #[test]
    fn test_non_parameter_where_with_lifetime() {
        let res = non_parameter_where_with_lifetime(&[1u8, 2]);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_non_parameter_where_with_lifetime_fake() {
        non_parameter_where_with_lifetime_fake::<u8>().setup(|items| items.len() + 1);

        let res = non_parameter_where_with_lifetime(&[1u8, 2]);
        assert_eq!(res, 3);
    }
}

mod spy {
    #[fnmock::spyable]
    fn non_parameter_where_with_lifetime<'a, T: 'static + Clone>(items: &'a [T]) -> usize
    where
        Vec<&'a T>: Clone,
    {
        items.len()
    }

    #[test]
    fn test_non_parameter_where_with_lifetime() {
        let spy = non_parameter_where_with_lifetime_spy::<u8>();
        spy.expectf(|items: &[u8]| items == [1u8, 2]).once();

        let res = non_parameter_where_with_lifetime(&[1u8, 2]);

        assert_eq!(res, 2);
        spy.assert();
    }
}

mod mock {
    #[fnmock::mockable]
    fn non_parameter_where_with_lifetime<'a, T: 'static + Clone>(items: &'a [T]) -> usize
    where
        Vec<&'a T>: Clone,
    {
        items.len()
    }

    #[test]
    fn test_non_parameter_where_with_lifetime() {
        let mock = non_parameter_where_with_lifetime_mock::<u8>();
        mock.setup(|items| items.len() + 1);
        mock.expectf(|items: &[u8]| items == [1u8, 2]).once();

        let res = non_parameter_where_with_lifetime(&[1u8, 2]);

        assert_eq!(res, 3);
        mock.assert();
    }
}
