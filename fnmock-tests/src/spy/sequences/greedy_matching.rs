#[fnmock::spyable]
fn greedy_fn(id: i32) {
    let _ = id;
}

/// Sequencing is greedy: an earlier step that can still accept calls consumes them even if a
/// later step matches the same arguments, which can starve the later step of calls it needed.
#[test]
#[should_panic(expected = "Expectation(s) of the spied function")]
fn test_earlier_unbounded_step_starves_a_later_step_matching_the_same_arguments() {
    let spy = greedy_fn_spy();
    let mut seq = fnmock::Sequence::new();
    // No `times`/`once`/`never`: unbounded, so it keeps accepting `(2)` forever.
    spy.expect(fnmock::predicate::eq(2)).in_sequence(&mut seq);
    spy.expect(fnmock::predicate::eq(2))
        .once()
        .in_sequence(&mut seq);

    greedy_fn(2);
    greedy_fn(2);
    greedy_fn(2); // all three are consumed by the first, unbounded step

    // The second step never got a call, even though every call matched its predicate too.
    spy.assert();
}
