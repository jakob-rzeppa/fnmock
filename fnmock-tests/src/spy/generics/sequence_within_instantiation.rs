#[fnmock::spyable]
fn sequence_within_instantiation<T: 'static>(a: T) {
    let _ = a;
}

#[test]
fn test_calls_in_order_fulfill_the_sequence() {
    let spy = sequence_within_instantiation_spy::<i32>();
    let mut seq = fnmock::Sequence::new();
    spy.expect(fnmock::predicate::eq(2))
        .times(2)
        .in_sequence(&mut seq);
    spy.expect(fnmock::predicate::eq(5))
        .once()
        .in_sequence(&mut seq);

    sequence_within_instantiation(2);
    sequence_within_instantiation(2);
    sequence_within_instantiation(5);

    spy.assert();
}

#[test]
#[should_panic(expected = "Call out of sequence")]
fn test_out_of_order_calls_panic_in_a_strict_sequence() {
    let spy = sequence_within_instantiation_spy::<i32>();
    let mut seq = fnmock::Sequence::new_strict();
    spy.expect(fnmock::predicate::eq(2))
        .once()
        .in_sequence(&mut seq);
    spy.expect(fnmock::predicate::eq(5))
        .once()
        .in_sequence(&mut seq);

    sequence_within_instantiation(5);
}
