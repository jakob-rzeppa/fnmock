//! `Sequence::default()` has to be the same thing as `Sequence::new()` — not `Sequence::strict()`.
//! This test ensures that the default sequence is lenient, not strict.

#[fnmock::spyable]
fn default_seq_target(id: i32) {
    let _ = id;
}

#[test]
fn test_default_sequence_is_lenient_not_strict() {
    let spy = default_seq_target_spy();
    let mut seq = fnmock::Sequence::default();
    spy.expect(fnmock::predicate::eq(1))
        .once()
        .in_sequence(&mut seq);
    spy.expect(fnmock::predicate::eq(2))
        .once()
        .in_sequence(&mut seq);

    default_seq_target(2);
    default_seq_target(1);
    default_seq_target(2);

    spy.assert();
}
