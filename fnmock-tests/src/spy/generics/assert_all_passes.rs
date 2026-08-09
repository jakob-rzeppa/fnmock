//! `f_spy_all()` sweeps every instantiation. It carries nothing but `assert`.

#[fnmock::spyable]
fn assert_all_passes<T: 'static>(a: T) {
    let _ = a;
}

#[test]
fn test_assert_all_passes_when_every_instantiation_is_satisfied() {
    let spy_string = assert_all_passes_spy::<String>();
    let spy_i32 = assert_all_passes_spy::<i32>();
    spy_string.expect(fnmock::predicate::always()).once();
    spy_i32.expect(fnmock::predicate::always()).times(2);

    assert_all_passes("hi".to_string());
    assert_all_passes(1);
    assert_all_passes(2);

    assert_all_passes_spy_all().assert();
}

#[test]
fn test_assert_all_passes_when_no_instantiation_was_ever_touched() {
    assert_all_passes_spy_all().assert();
}
