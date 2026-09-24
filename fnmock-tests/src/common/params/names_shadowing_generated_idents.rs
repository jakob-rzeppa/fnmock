//! Parameter names that collide with identifiers the generated code binds for itself: the
//! `Display` impl's formatter, `Matcher::matches`'s params argument, and the matcher enum's
//! `Function` field.

mod fake {
    #[fnmock::fakeable]
    fn names_shadowing_generated_idents(f: u32, params: u32, function: u32) -> u32 {
        f + params + function
    }

    #[test]
    fn test_names_shadowing_generated_idents() {
        let res = names_shadowing_generated_idents(1, 2, 3);
        assert_eq!(res, 6);
    }

    #[test]
    fn test_names_shadowing_generated_idents_fake() {
        names_shadowing_generated_idents_fake().setup(|f, params, function| f + params + function);

        let res = names_shadowing_generated_idents(1, 2, 3);
        assert_eq!(res, 6);
    }
}

mod spy {
    #[fnmock::spyable]
    fn names_shadowing_generated_idents(f: u32, params: u32, function: u32) -> u32 {
        f + params + function
    }

    #[test]
    fn test_names_shadowing_generated_idents() {
        let spy = names_shadowing_generated_idents_spy();
        spy.expect(
            fnmock::predicate::eq(1),
            fnmock::predicate::eq(2),
            fnmock::predicate::eq(3),
        )
        .once();

        let res = names_shadowing_generated_idents(1, 2, 3);

        assert_eq!(res, 6);
        spy.assert();
    }

    #[test]
    #[should_panic(expected = "f == 1 && params == 2 && function == 3")]
    fn test_matcher_display_names_every_parameter() {
        let spy = names_shadowing_generated_idents_spy();
        spy.expect(
            fnmock::predicate::eq(1),
            fnmock::predicate::eq(2),
            fnmock::predicate::eq(3),
        )
        .once();

        spy.assert();
    }
}

mod mock {
    #[fnmock::mockable]
    fn names_shadowing_generated_idents(f: u32, params: u32, function: u32) -> u32 {
        f + params + function
    }

    #[test]
    fn test_names_shadowing_generated_idents() {
        let mock = names_shadowing_generated_idents_mock();
        mock.setup(|f, params, function| f * params * function);
        mock.expect(
            fnmock::predicate::eq(1),
            fnmock::predicate::eq(2),
            fnmock::predicate::eq(3),
        )
        .once();

        let res = names_shadowing_generated_idents(1, 2, 3);

        assert_eq!(res, 6);
        mock.assert();
    }
}
