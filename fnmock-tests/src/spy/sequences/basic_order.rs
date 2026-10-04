mod spy {
    #[fnmock::spyable]
    fn sequenced_fn(id: i32) {
        let _ = id;
    }

    #[test]
    fn test_calls_in_order_fulfill_sequence() {
        let spy = sequenced_fn_spy();
        let seq = fnmock::Sequence::new();
        spy.expect(fnmock::predicate::eq(2))
            .times(3)
            .in_sequence(&seq);
        spy.expect(fnmock::predicate::eq(5))
            .once()
            .in_sequence(&seq);

        sequenced_fn(2);
        sequenced_fn(2);
        sequenced_fn(2);
        sequenced_fn(5);

        spy.assert();
    }
}

mod mock {
    #[fnmock::mockable]
    fn sequenced_fn(id: i32) {
        let _ = id;
    }

    #[test]
    fn test_calls_in_order_fulfill_sequence() {
        let mock = sequenced_fn_mock();
        let seq = fnmock::Sequence::new();
        mock.expect(fnmock::predicate::eq(2))
            .times(3)
            .in_sequence(&seq);
        mock.expect(fnmock::predicate::eq(5))
            .once()
            .in_sequence(&seq);

        sequenced_fn(2);
        sequenced_fn(2);
        sequenced_fn(2);
        sequenced_fn(5);

        mock.assert();
    }
}
