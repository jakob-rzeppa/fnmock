//! `&T` has its reference stripped like any other parameter, so
//! the predicate is a `Predicate<T>` rather than a `Predicate<&T>`.

#[fnmock::spyable]
fn generic_reference_param<T: 'static>(a: &T) {
    let _ = a;
}

#[test]
fn test_generic_reference_param() {
    let spy = generic_reference_param_spy::<String>();
    spy.expect(fnmock::predicate::eq("hi".to_string())).once();

    generic_reference_param(&"hi".to_string());

    spy.assert();
}

#[test]
fn test_generic_reference_param_isolation() {
    let spy_string = generic_reference_param_spy::<String>();
    let spy_i32 = generic_reference_param_spy::<i32>();
    spy_string.expect_once();
    spy_i32.expect_once();

    generic_reference_param(&"hi".to_string());
    generic_reference_param(&2);

    spy_string.assert();
    spy_i32.assert();
}
