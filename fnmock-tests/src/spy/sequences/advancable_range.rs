#[fnmock::spyable]
fn advancable_fn(id: i32) {
    let _ = id;
}

/// A ranged step becomes advancable once its minimum is reached, not only once its maximum is —
/// so the sequence can move on to the next step well before the ranged step's maximum.
#[test]
fn test_ranged_step_advances_after_reaching_its_minimum() {
    let spy = advancable_fn_spy();
    let mut seq = fnmock::Sequence::new();
    spy.expect(fnmock::predicate::eq(2))
        .times(1..4)
        .in_sequence(&mut seq);
    spy.expect(fnmock::predicate::eq(5))
        .once()
        .in_sequence(&mut seq);

    advancable_fn(2); // reaches the minimum of 1..4, making the step advancable
    advancable_fn(5); // advances past it even though its maximum was never reached

    spy.assert();
}
