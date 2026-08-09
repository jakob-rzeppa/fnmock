//! The full expectation surface has to work on a generic spy exactly as it
//! does on a plain one, scoped to the instantiation it was set on.

#[fnmock::spyable]
fn expectations_per_instantiation<T: 'static>(a: T) {
    let _ = a;
}

#[test]
fn test_times_on_one_instantiation() {
    let spy = expectations_per_instantiation_spy::<i32>();
    spy.expect(fnmock::predicate::eq(2)).times(1..=3);

    expectations_per_instantiation(2);
    expectations_per_instantiation(2);

    spy.assert();
}

#[test]
fn test_expectf_on_one_instantiation() {
    let spy = expectations_per_instantiation_spy::<i32>();
    spy.expectf(|a: &i32| *a > 10).times(2);

    expectations_per_instantiation(11);
    expectations_per_instantiation(12);
    expectations_per_instantiation(1);

    spy.assert();
}

#[test]
#[should_panic(expected = "large values")]
fn test_describe_names_the_expectation_in_the_failure() {
    let spy = expectations_per_instantiation_spy::<i32>();
    spy.expectf(|a: &i32| *a > 10)
        .describe("large values".to_string())
        .times(2);

    expectations_per_instantiation(11);

    spy.assert();
}

#[test]
fn test_multiple_independent_expectations_on_one_instantiation() {
    let spy = expectations_per_instantiation_spy::<i32>();
    spy.expect(fnmock::predicate::eq(2)).once();
    spy.expect(fnmock::predicate::eq(5)).once();

    expectations_per_instantiation(2);
    expectations_per_instantiation(5);

    spy.assert();
}

#[test]
fn test_a_call_no_expectation_matches_is_not_an_error() {
    let spy = expectations_per_instantiation_spy::<i32>();
    spy.expect(fnmock::predicate::eq(2)).once();

    expectations_per_instantiation(2);
    expectations_per_instantiation(99);

    spy.assert();
}
