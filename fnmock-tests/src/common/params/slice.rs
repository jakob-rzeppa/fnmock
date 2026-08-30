mod fake {
    #[fnmock::fakeable]
    fn slice(items: &[i32]) -> i32 {
        items.iter().sum()
    }

    #[test]
    fn test_slice() {
        assert_eq!(slice(&[2, 3, 4]), 9);
    }

    #[test]
    fn test_slice_fake() {
        slice_fake().setup(|items: &[i32]| items.iter().product());

        assert_eq!(slice(&[2, 3, 4]), 24);
    }
}

mod spy {
    #[fnmock::spyable]
    fn slice(items: &[i32]) -> i32 {
        items.iter().sum()
    }

    #[test]
    fn test_slice() {
        let spy = slice_spy();
        spy.expect(fnmock::predicate::eq(&[2, 3, 4][..]));

        slice(&[2, 3, 4]);

        spy.assert();
    }
}
