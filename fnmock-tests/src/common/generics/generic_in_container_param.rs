//! The parameter types mention `T` without being `T`, so the matcher's
//! predicates are over the container types.

mod fake {
    #[fnmock::fakeable]
    fn generic_in_container_param<T: 'static>(items: Vec<T>, first: Option<T>) -> Vec<T> {
        let mut items = items;
        items.extend(first);
        items
    }

    #[test]
    fn test_generic_in_container_param() {
        let res = generic_in_container_param(vec![1, 2, 3], Some(4));
        assert_eq!(res, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_generic_in_container_param_fake() {
        generic_in_container_param_fake::<i32>().setup(|items, _first| items);

        let res = generic_in_container_param(vec![1, 2, 3], Some(4));

        assert_eq!(res, vec![1, 2, 3]);
    }
}

mod spy {
    #[fnmock::spyable]
    fn generic_in_container_param<T: 'static>(items: Vec<T>, first: Option<T>) {
        let _ = (items, first);
    }

    #[test]
    fn test_generic_in_container_param() {
        let spy = generic_in_container_param_spy::<i32>();
        spy.expect(
            fnmock::predicate::eq(vec![1, 2, 3]),
            fnmock::predicate::eq(Some(1)),
        )
        .once();

        generic_in_container_param(vec![1, 2, 3], Some(1));

        spy.assert();
    }

    #[test]
    fn test_generic_in_container_param_isolation() {
        let spy_i32 = generic_in_container_param_spy::<i32>();
        let spy_string = generic_in_container_param_spy::<String>();
        spy_i32.expect_once();
        spy_string.expect_never();

        generic_in_container_param(vec![1, 2, 3], Some(1));

        spy_i32.assert();
        spy_string.assert();
    }
}

mod mock {
    #[fnmock::mockable]
    fn generic_in_container_param<T: 'static>(items: Vec<T>, first: Option<T>) -> Vec<T> {
        let mut items = items;
        items.extend(first);
        items
    }

    #[test]
    fn test_generic_in_container_param() {
        let mock = generic_in_container_param_mock::<i32>();
        mock.setup(|items, _first| items);
        mock.expect(
            fnmock::predicate::eq(vec![1, 2, 3]),
            fnmock::predicate::eq(Some(1)),
        )
        .once();

        let res = generic_in_container_param(vec![1, 2, 3], Some(1));

        assert_eq!(res, vec![1, 2, 3]);
        mock.assert();
    }
}
