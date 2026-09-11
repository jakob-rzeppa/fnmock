mod fake {
    #[fnmock::fakeable]
    fn reference_in_vec(items: Vec<&i32>) -> i32 {
        items.iter().map(|e| (*e).clone()).sum()
    }

    #[test]
    fn test_reference_in_vec() {
        assert_eq!(reference_in_vec(vec![&2, &3, &4]), 9);
    }

    #[test]
    fn test_reference_in_vec_fake() {
        reference_in_vec_fake()
            .setup(|items: Vec<&i32>| items.iter().map(|e| (*e).clone()).product());

        assert_eq!(reference_in_vec(vec![&2, &3, &4]), 24);
    }
}

mod spy {
    #[fnmock::spyable]
    fn reference_in_vec(items: Vec<&'static i32>) -> i32 {
        items.iter().map(|e| (*e).clone()).sum()
    }

    #[test]
    fn test_reference_in_vec() {
        let spy = reference_in_vec_spy();
        spy.expect(fnmock::predicate::eq(vec![&2, &3, &4]));

        reference_in_vec(vec![&2, &3, &4]);

        spy.assert();
    }
}
