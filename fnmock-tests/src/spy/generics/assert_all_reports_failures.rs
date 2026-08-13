//! The point of `f_spy_all()` is that a forgotten per-instantiation `assert`
//! cannot silently pass, so the failure has to name the instantiation it came
//! from.

#[fnmock::spyable]
fn assert_all_reports_failures<T: 'static>(a: T) {
    let _ = a;
}

#[test]
#[should_panic(expected = "assert_all_reports_failures::<i32>")]
fn test_assert_all_names_the_failing_instantiation() {
    let spy_string = assert_all_reports_failures_spy::<String>();
    let spy_i32 = assert_all_reports_failures_spy::<i32>();
    spy_string.expect(fnmock::predicate::always()).once();
    spy_i32.expect(fnmock::predicate::always()).once();

    // Only the String instantiation is satisfied.
    assert_all_reports_failures("hi".to_string());

    assert_all_reports_failures_spy_all().assert();
}

#[test]
#[should_panic(expected = "alloc::string::String")]
fn test_assert_all_reports_every_failing_instantiation_at_once() {
    let spy_string = assert_all_reports_failures_spy::<String>();
    let spy_i32 = assert_all_reports_failures_spy::<i32>();
    spy_string.expect(fnmock::predicate::always()).once();
    spy_i32.expect(fnmock::predicate::always()).once();

    assert_all_reports_failures_spy_all().assert();
}

#[test]
#[should_panic(expected = "assert_all_reports_failures_const::<5>")]
fn test_assert_all_names_a_const_instantiation_by_value() {
    let spy = assert_all_reports_failures_const_spy::<5>();
    spy.expect(fnmock::predicate::always()).once();

    assert_all_reports_failures_const_spy_all().assert();
}

#[fnmock::spyable]
fn assert_all_reports_failures_const<const C: usize>(a: i32) {
    let _ = (a, C);
}
