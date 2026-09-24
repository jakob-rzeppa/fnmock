//! `.expect()` is generated as a working, predicate-based method for every spy whose parameters
//! all name no lifetime other than `'static` after elision. For every other spy it is generated
//! as a `#[deprecated]`, zero-argument stub that panics via `unimplemented!()` if called; see
//! `type_needs_a_lifetime_outside_fn_sugar` in `fnmock-derive`.
//!
//! This file exercises both sides of that split: every shape below that should disable `.expect()`
//! panics with the stub's message when called, and every shape that should keep it works exactly
//! like an ordinary predicate-based expectation. The compile-time `#[deprecated]` note is checked
//! separately by the `.cf.rs`/`.cf.stderr` fixtures next to this file.

mod spy {
    struct Ref<'a>(&'a str);

    mod unsupported {
        use super::Ref;

        #[fnmock::spyable]
        fn owned_lifetime_param(r: Ref<'_>) -> usize {
            r.0.len()
        }

        #[test]
        #[should_panic(
            expected = "`.expect()` is not available on this spy; use `.expectf()` instead"
        )]
        fn test_expect_panics_for_a_lifetime_parameterized_struct_by_value() {
            let spy = owned_lifetime_param_spy();
            #[allow(deprecated)]
            spy.expect();
        }

        #[fnmock::spyable]
        fn reference_to_lifetime_param(r: &Ref<'_>) -> usize {
            r.0.len()
        }

        #[test]
        #[should_panic(
            expected = "`.expect()` is not available on this spy; use `.expectf()` instead"
        )]
        fn test_expect_panics_for_a_reference_to_a_lifetime_parameterized_struct() {
            // Stripping the outer `&` still leaves `Ref<'_>`'s own lifetime behind, so this stays
            // unsupported even though a bare `&str` (below, in `supported`) is fine.
            let spy = reference_to_lifetime_param_spy();
            #[allow(deprecated)]
            spy.expect();
        }

        #[fnmock::spyable]
        fn lifetime_with_generic<'a, T: 'static>(r: Ref<'a>, value: T) -> usize {
            let _ = value;
            r.0.len()
        }

        #[test]
        #[should_panic(
            expected = "`.expect()` is not available on this spy; use `.expectf()` instead"
        )]
        fn test_expect_panics_when_a_lifetime_is_mixed_with_a_generic() {
            let spy = lifetime_with_generic_spy::<i32>();
            #[allow(deprecated)]
            spy.expect();
        }

        #[fnmock::spyable]
        fn two_named_lifetimes<'a, 'b>(left: Ref<'a>, right: Ref<'b>) -> usize {
            left.0.len() + right.0.len()
        }

        #[test]
        #[should_panic(
            expected = "`.expect()` is not available on this spy; use `.expectf()` instead"
        )]
        fn test_expect_panics_for_multiple_named_lifetimes() {
            let spy = two_named_lifetimes_spy();
            #[allow(deprecated)]
            spy.expect();
        }

        #[fnmock::spyable]
        fn nested_reference_in_slice<'a>(items: &'a [&'a str]) -> usize {
            items.len()
        }

        #[test]
        #[should_panic(
            expected = "`.expect()` is not available on this spy; use `.expectf()` instead"
        )]
        fn test_expect_panics_for_a_reference_nested_in_a_slice() {
            let spy = nested_reference_in_slice_spy();
            #[allow(deprecated)]
            spy.expect();
        }

        /// The panic message is exactly the `unimplemented!()` payload -- `unimplemented!` prepends
        /// "not implemented: " to it, so this checks the payload is unmodified rather than relying on
        /// `#[should_panic]`'s substring match alone.
        #[test]
        fn test_panic_message_is_exactly_the_unimplemented_payload() {
            let spy = owned_lifetime_param_spy();
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                #[allow(deprecated)]
                spy.expect()
            }));

            let Err(panic_payload) = result else {
                panic!("expected spy.expect() to panic");
            };
            let payload = *panic_payload.downcast::<&str>().unwrap();
            assert_eq!(
                payload,
                "not implemented: `.expect()` is not available on this spy; use `.expectf()` instead"
            );
        }

        /// Only the predicate-based `expect()` is disabled -- the rest of the expectation surface
        /// keeps working on a spy that can't support it.
        #[test]
        fn test_expectf_and_expect_times_still_work_when_expect_is_unavailable() {
            let spy = owned_lifetime_param_spy();
            spy.expectf(|r: &Ref<'_>| r.0 == "hi").once();

            let owned = "hi".to_string();
            owned_lifetime_param(Ref(&owned));

            spy.assert();
        }
    }

    mod supported {
        #[fnmock::spyable]
        fn no_lifetime_at_all(id: i32) -> i32 {
            id
        }

        #[test]
        fn test_expect_works_with_no_lifetime_in_the_signature() {
            let spy = no_lifetime_at_all_spy();
            spy.expect(fnmock::predicate::eq(2)).once();

            no_lifetime_at_all(2);

            spy.assert();
        }

        #[fnmock::spyable]
        fn named_lifetime_reference<'a>(s: &'a str) -> usize {
            s.len()
        }

        #[test]
        fn test_expect_works_for_a_named_lifetime_on_a_plain_reference() {
            let spy = named_lifetime_reference_spy();
            spy.expect(fnmock::predicate::eq("hi".to_string())).once();

            named_lifetime_reference("hi");

            spy.assert();
        }

        #[fnmock::spyable]
        fn elided_lifetime_reference(s: &str) -> usize {
            s.len()
        }

        #[test]
        fn test_expect_works_for_an_elided_lifetime_on_a_plain_reference() {
            let spy = elided_lifetime_reference_spy();
            spy.expect(fnmock::predicate::eq("hi".to_string())).once();

            elided_lifetime_reference("hi");

            spy.assert();
        }

        #[fnmock::spyable]
        fn generic_without_lifetime<T: 'static>(value: T) -> T {
            value
        }

        #[test]
        fn test_expect_works_for_a_generic_instantiation_without_a_lifetime() {
            let spy = generic_without_lifetime_spy::<i32>();
            spy.expect(fnmock::predicate::eq(2)).once();

            generic_without_lifetime(2);

            spy.assert();
        }
    }
}

mod mock {
    struct Ref<'a>(&'a str);

    mod unsupported {
        use super::Ref;

        #[fnmock::mockable]
        fn owned_lifetime_param(r: Ref<'_>) -> usize {
            r.0.len()
        }

        #[test]
        #[should_panic(
            expected = "`.expect()` is not available on this spy; use `.expectf()` instead"
        )]
        fn test_expect_panics_for_a_lifetime_parameterized_struct_by_value() {
            let mock = owned_lifetime_param_mock();
            #[allow(deprecated)]
            mock.expect();
        }

        #[fnmock::mockable]
        fn reference_to_lifetime_param(r: &Ref<'_>) -> usize {
            r.0.len()
        }

        #[test]
        #[should_panic(
            expected = "`.expect()` is not available on this spy; use `.expectf()` instead"
        )]
        fn test_expect_panics_for_a_reference_to_a_lifetime_parameterized_struct() {
            // Stripping the outer `&` still leaves `Ref<'_>`'s own lifetime behind, so this stays
            // unsupported even though a bare `&str` (below, in `supported`) is fine.
            let mock = reference_to_lifetime_param_mock();
            #[allow(deprecated)]
            mock.expect();
        }

        #[fnmock::mockable]
        fn lifetime_with_generic<'a, T: 'static>(r: Ref<'a>, value: T) -> usize {
            let _ = value;
            r.0.len()
        }

        #[test]
        #[should_panic(
            expected = "`.expect()` is not available on this spy; use `.expectf()` instead"
        )]
        fn test_expect_panics_when_a_lifetime_is_mixed_with_a_generic() {
            let mock = lifetime_with_generic_mock::<i32>();
            #[allow(deprecated)]
            mock.expect();
        }

        #[fnmock::mockable]
        fn two_named_lifetimes<'a, 'b>(left: Ref<'a>, right: Ref<'b>) -> usize {
            left.0.len() + right.0.len()
        }

        #[test]
        #[should_panic(
            expected = "`.expect()` is not available on this spy; use `.expectf()` instead"
        )]
        fn test_expect_panics_for_multiple_named_lifetimes() {
            let mock = two_named_lifetimes_mock();
            #[allow(deprecated)]
            mock.expect();
        }

        #[fnmock::mockable]
        fn nested_reference_in_slice<'a>(items: &'a [&'a str]) -> usize {
            items.len()
        }

        #[test]
        #[should_panic(
            expected = "`.expect()` is not available on this spy; use `.expectf()` instead"
        )]
        fn test_expect_panics_for_a_reference_nested_in_a_slice() {
            let mock = nested_reference_in_slice_mock();
            #[allow(deprecated)]
            mock.expect();
        }

        /// The panic message is exactly the `unimplemented!()` payload -- `unimplemented!` prepends
        /// "not implemented: " to it, so this checks the payload is unmodified rather than relying on
        /// `#[should_panic]`'s substring match alone.
        #[test]
        fn test_panic_message_is_exactly_the_unimplemented_payload() {
            let mock = owned_lifetime_param_mock();
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                #[allow(deprecated)]
                mock.expect()
            }));

            let Err(panic_payload) = result else {
                panic!("expected mock.expect() to panic");
            };
            let payload = *panic_payload.downcast::<&str>().unwrap();
            assert_eq!(
                payload,
                "not implemented: `.expect()` is not available on this spy; use `.expectf()` instead"
            );
        }

        /// Only the predicate-based `expect()` is disabled -- the rest of the expectation surface
        /// keeps working on a spy that can't support it.
        #[test]
        fn test_expectf_and_expect_times_still_work_when_expect_is_unavailable() {
            let mock = owned_lifetime_param_mock();
            mock.expectf(|r: &Ref<'_>| r.0 == "hi").once();

            let owned = "hi".to_string();
            owned_lifetime_param(Ref(&owned));

            mock.assert();
        }
    }

    mod supported {
        #[fnmock::mockable]
        fn no_lifetime_at_all(id: i32) -> i32 {
            id
        }

        #[test]
        fn test_expect_works_with_no_lifetime_in_the_signature() {
            let mock = no_lifetime_at_all_mock();
            mock.expect(fnmock::predicate::eq(2)).once();

            no_lifetime_at_all(2);

            mock.assert();
        }

        #[fnmock::mockable]
        fn named_lifetime_reference<'a>(s: &'a str) -> usize {
            s.len()
        }

        #[test]
        fn test_expect_works_for_a_named_lifetime_on_a_plain_reference() {
            let mock = named_lifetime_reference_mock();
            mock.expect(fnmock::predicate::eq("hi".to_string())).once();

            named_lifetime_reference("hi");

            mock.assert();
        }

        #[fnmock::mockable]
        fn elided_lifetime_reference(s: &str) -> usize {
            s.len()
        }

        #[test]
        fn test_expect_works_for_an_elided_lifetime_on_a_plain_reference() {
            let mock = elided_lifetime_reference_mock();
            mock.expect(fnmock::predicate::eq("hi".to_string())).once();

            elided_lifetime_reference("hi");

            mock.assert();
        }

        #[fnmock::mockable]
        fn generic_without_lifetime<T: 'static>(value: T) -> T {
            value
        }

        #[test]
        fn test_expect_works_for_a_generic_instantiation_without_a_lifetime() {
            let mock = generic_without_lifetime_mock::<i32>();
            mock.expect(fnmock::predicate::eq(2)).once();

            generic_without_lifetime(2);

            mock.assert();
        }
    }
}
