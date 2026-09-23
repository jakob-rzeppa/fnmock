#[fnmock::spyable]
fn clear_generic<T: 'static>(a: T) {
    let _ = a;
}

#[test]
fn test_clear_resets_its_own_instantiation() {
    let spy = clear_generic_spy::<String>();
    spy.expect(fnmock::predicate::always()).once();

    spy.clear();

    spy.assert();
}

#[test]
fn test_clear_leaves_other_instantiations_expectations() {
    let spy_string = clear_generic_spy::<String>();
    let spy_i32 = clear_generic_spy::<i32>();
    spy_string.expect(fnmock::predicate::always()).once();
    spy_i32.expect(fnmock::predicate::always()).once();

    spy_string.clear();
    clear_generic(1i32);

    spy_string.assert();
    spy_i32.assert();
}

#[test]
#[should_panic(expected = "Expectation(s) of the spied function")]
fn test_other_instantiation_is_still_checked_after_clear() {
    let spy_string = clear_generic_spy::<String>();
    let spy_i32 = clear_generic_spy::<i32>();
    spy_string.expect(fnmock::predicate::always()).once();
    spy_i32.expect(fnmock::predicate::always()).once();

    spy_string.clear();

    spy_i32.assert();
}

#[test]
fn test_clear_leaves_other_instantiations_call_history() {
    let spy_string = clear_generic_spy::<String>();
    let spy_i32 = clear_generic_spy::<i32>();
    spy_i32.expect_times(2);
    clear_generic(1i32);

    spy_string.clear();
    clear_generic(2i32);

    spy_i32.assert();
}

#[test]
fn test_clear_on_an_untouched_instantiation_is_a_noop() {
    clear_generic_spy::<u8>().clear();

    clear_generic_spy::<u8>().assert();
}

#[test]
fn test_clear_removes_only_its_instantiations_sequence_steps() {
    let spy_string = clear_generic_spy::<String>();
    let spy_i32 = clear_generic_spy::<i32>();
    let mut seq = fnmock::Sequence::new_strict();
    spy_string
        .expect(fnmock::predicate::always())
        .once()
        .in_sequence(&mut seq);
    spy_i32
        .expect(fnmock::predicate::always())
        .once()
        .in_sequence(&mut seq);

    spy_string.clear();

    clear_generic(1i32);

    spy_i32.assert();
}
