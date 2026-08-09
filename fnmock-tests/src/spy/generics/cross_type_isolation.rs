//! Expectations set on one instantiation must not see calls made with other
//! generic arguments.

#[fnmock::spyable]
fn cross_type_isolation<T: 'static>(a: T) -> T {
    a
}

#[test]
fn test_expectations_do_not_leak_across_instantiations() {
    let spy_string = cross_type_isolation_spy::<String>();
    let spy_i32 = cross_type_isolation_spy::<i32>();
    spy_string.expect(fnmock::predicate::always()).once();
    spy_i32.expect(fnmock::predicate::always()).times(2);

    cross_type_isolation("hi".to_string());
    cross_type_isolation(1);
    cross_type_isolation(2);

    spy_string.assert();
    spy_i32.assert();
}

#[test]
fn test_global_times_is_per_instantiation() {
    let spy_string = cross_type_isolation_spy::<String>();
    let spy_i32 = cross_type_isolation_spy::<i32>();
    spy_string.expect_times(1);
    spy_i32.expect_times(2);

    cross_type_isolation("hi".to_string());
    cross_type_isolation(1);
    cross_type_isolation(2);

    spy_string.assert();
    spy_i32.assert();
}

/// The total call range is enforced per instantiation, so the second `i32`
/// call is one too many even though the `String` instantiation is untouched.
#[test]
#[should_panic(expected = "Too many calls of the spied function")]
fn test_exceeding_one_instantiations_total_calls_panics() {
    let spy_i32 = cross_type_isolation_spy::<i32>();
    spy_i32.expect_times(1);

    cross_type_isolation("hi".to_string());
    cross_type_isolation("there".to_string());
    cross_type_isolation(1);
    cross_type_isolation(2);
}

/// Reaching the same instantiation twice must return a view onto the same
/// store, not a fresh one.
#[test]
fn test_the_accessor_reaches_the_same_store_each_time() {
    cross_type_isolation_spy::<u8>().expect_times(2);

    cross_type_isolation(1u8);
    cross_type_isolation(2u8);

    cross_type_isolation_spy::<u8>().assert();
}
