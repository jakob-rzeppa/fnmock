//! A sequence stores its steps as `dyn DynExpectation` and downcasts to
//! `Expectation<M>` to decide whether a call belongs to a step. `Matcher<i32>`
//! and `Matcher<String>` are distinct types, so instantiations of the *same*
//! function disambiguate the same way two different functions already do.

#[fnmock::spyable]
fn sequence_across_instantiations<T: 'static>(a: T) {
    let _ = a;
}

#[test]
fn test_sequence_spans_two_instantiations() {
    let spy_i32 = sequence_across_instantiations_spy::<i32>();
    let spy_string = sequence_across_instantiations_spy::<String>();
    let mut seq = fnmock::Sequence::new();
    spy_i32
        .expect(fnmock::predicate::eq(2))
        .once()
        .in_sequence(&mut seq);
    spy_string
        .expect(fnmock::predicate::eq("hi".to_string()))
        .once()
        .in_sequence(&mut seq);

    sequence_across_instantiations(2);
    sequence_across_instantiations("hi".to_string());

    spy_i32.assert();
    spy_string.assert();
}

#[test]
#[should_panic(expected = "Call out of sequence")]
fn test_strict_sequence_panics_when_instantiations_are_called_in_the_wrong_order() {
    let spy_i32 = sequence_across_instantiations_spy::<i32>();
    let spy_string = sequence_across_instantiations_spy::<String>();
    let mut seq = fnmock::Sequence::new_strict();
    spy_i32
        .expect(fnmock::predicate::eq(2))
        .once()
        .in_sequence(&mut seq);
    spy_string
        .expect(fnmock::predicate::eq("hi".to_string()))
        .once()
        .in_sequence(&mut seq);

    // The i32 instantiation must be called first.
    sequence_across_instantiations("hi".to_string());
}

/// A call on one instantiation must not be able to satisfy a sequence step
/// belonging to another, even when the predicate would otherwise match.
#[test]
#[should_panic(expected = "Expectation(s) of the spied function")]
fn test_a_call_on_one_instantiation_does_not_advance_another_ones_step() {
    let spy_i32 = sequence_across_instantiations_spy::<i32>();
    let spy_u8 = sequence_across_instantiations_spy::<u8>();
    let mut seq = fnmock::Sequence::new();
    spy_u8
        .expect(fnmock::predicate::always())
        .once()
        .in_sequence(&mut seq);

    sequence_across_instantiations(2i32);

    let _ = &spy_i32;
    spy_u8.assert();
}
