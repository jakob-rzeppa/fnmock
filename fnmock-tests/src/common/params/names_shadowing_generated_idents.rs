//! Parameter names that collide with identifiers the generated code binds for itself: the
//! `Display` impl's formatter, `Matcher::matches`'s params argument, and the matcher enum's
//! `Function` field, and the local the fake's inline call binds the fake `implementation` to.

mod fake {
    #[fnmock::fakeable]
    fn names_shadowing_generated_idents(
        f: u32,
        params: u32,
        function: u32,
        implementation: u32,
    ) -> u32 {
        f + params + function + implementation
    }

    #[test]
    fn test_names_shadowing_generated_idents() {
        let res = names_shadowing_generated_idents(1, 2, 3, 4);
        assert_eq!(res, 10);
    }

    #[test]
    fn test_names_shadowing_generated_idents_fake() {
        names_shadowing_generated_idents_fake()
            .setup(|f, params, function, implementation| f + params + function + implementation);

        let res = names_shadowing_generated_idents(1, 2, 3, 4);
        assert_eq!(res, 10);
    }
}

mod spy {
    #[fnmock::spyable]
    fn names_shadowing_generated_idents(
        f: u32,
        params: u32,
        function: u32,
        implementation: u32,
    ) -> u32 {
        f + params + function + implementation
    }

    #[test]
    fn test_names_shadowing_generated_idents() {
        let spy = names_shadowing_generated_idents_spy();
        spy.expect(
            fnmock::predicate::eq(1),
            fnmock::predicate::eq(2),
            fnmock::predicate::eq(3),
            fnmock::predicate::eq(4),
        )
        .once();

        let res = names_shadowing_generated_idents(1, 2, 3, 4);

        assert_eq!(res, 10);
        spy.assert();
    }

    #[test]
    #[should_panic(expected = "f == 1 && params == 2 && function == 3 && implementation == 4")]
    fn test_matcher_display_names_every_parameter() {
        let spy = names_shadowing_generated_idents_spy();
        spy.expect(
            fnmock::predicate::eq(1),
            fnmock::predicate::eq(2),
            fnmock::predicate::eq(3),
            fnmock::predicate::eq(4),
        )
        .once();

        spy.assert();
    }
}

mod mock {
    #[fnmock::mockable]
    fn names_shadowing_generated_idents(
        f: u32,
        params: u32,
        function: u32,
        implementation: u32,
    ) -> u32 {
        f + params + function + implementation
    }

    #[test]
    fn test_names_shadowing_generated_idents() {
        let mock = names_shadowing_generated_idents_mock();
        mock.setup(|f, params, function, implementation| f + params + function + implementation);
        mock.expect(
            fnmock::predicate::eq(1),
            fnmock::predicate::eq(2),
            fnmock::predicate::eq(3),
            fnmock::predicate::eq(4),
        )
        .once();

        let res = names_shadowing_generated_idents(1, 2, 3, 4);

        assert_eq!(res, 10);
        mock.assert();
    }
}
