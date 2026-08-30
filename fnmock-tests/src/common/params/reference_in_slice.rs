mod fake {
    #[fnmock::fakeable]
    fn reference_in_slice(items: &[&i32]) -> i32 {
        items.iter().map(|e| (*e).clone()).sum()
    }

    #[test]
    fn test_reference_in_slice() {
        assert_eq!(reference_in_slice(&[&2, &3, &4]), 9);
    }

    #[test]
    fn test_reference_in_slice_fake() {
        reference_in_slice_fake()
            .setup(|items: &[&i32]| items.iter().map(|e| (*e).clone()).product());

        assert_eq!(reference_in_slice(&[&2, &3, &4]), 24);
    }
}

mod spy {
    #[fnmock::spyable]
    fn reference_in_slice(items: &[&'static i32]) -> i32 {
        items.iter().map(|e| (*e).clone()).sum()
    }

    #[test]
    fn test_reference_in_slice() {
        let spy = reference_in_slice_spy();
        spy.expect(fnmock::predicate::eq(&[&2, &3, &4][..]));

        reference_in_slice(&[&2, &3, &4]);

        spy.assert();
    }
}
