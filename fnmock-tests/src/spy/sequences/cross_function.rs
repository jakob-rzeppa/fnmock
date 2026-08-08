#[fnmock::spyable]
fn cross_fn_a(id: &str) {
    let _ = id;
}

#[fnmock::spyable]
fn cross_fn_b(id: &str) {
    let _ = id;
}

#[test]
fn test_sequence_spans_two_functions() {
    let spy_a = cross_fn_a_spy();
    let spy_b = cross_fn_b_spy();
    let mut seq = fnmock::Sequence::new();
    spy_a
        .expect(fnmock::predicate::eq("a".to_string()))
        .once()
        .in_sequence(&mut seq);
    spy_b
        .expect(fnmock::predicate::eq("a".to_string()))
        .once()
        .in_sequence(&mut seq);

    cross_fn_a("a");
    cross_fn_b("a");

    spy_a.assert();
    spy_b.assert();
}

#[test]
#[should_panic(expected = "Call out of sequence")]
fn test_strict_cross_function_sequence_panics_on_wrong_function_order() {
    let spy_a = cross_fn_a_spy();
    let spy_b = cross_fn_b_spy();
    let mut seq = fnmock::Sequence::new_strict();
    spy_a
        .expect(fnmock::predicate::eq("a".to_string()))
        .once()
        .in_sequence(&mut seq);
    spy_b
        .expect(fnmock::predicate::eq("a".to_string()))
        .once()
        .in_sequence(&mut seq);

    cross_fn_b("a"); // wrong order: cross_fn_a must be called first
}
