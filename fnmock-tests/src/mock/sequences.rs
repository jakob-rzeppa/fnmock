//! A mock joins a `Sequence` and interleaves with plain spies and other mocks.

#[fnmock::mockable]
fn seq_mock(id: i32) -> i32 {
    id
}

#[fnmock::spyable]
fn seq_plain_spy(id: i32) {
    let _ = id;
}

#[fnmock::mockable]
fn seq_other_mock(id: i32) {
    let _ = id;
}

#[test]
fn test_mock_and_plain_spy_interleave_in_one_sequence() {
    let mock = seq_mock_mock();
    let spy = seq_plain_spy_spy();
    let mut seq = fnmock::Sequence::new_strict();
    mock.setup(|id| id * 2);
    mock.expect(fnmock::predicate::eq(1))
        .once()
        .in_sequence(&mut seq);
    spy.expect(fnmock::predicate::eq(2))
        .once()
        .in_sequence(&mut seq);
    mock.expect(fnmock::predicate::eq(3))
        .once()
        .in_sequence(&mut seq);

    assert_eq!(seq_mock(1), 2);
    seq_plain_spy(2);
    assert_eq!(seq_mock(3), 6);

    mock.assert();
    spy.assert();
}

#[test]
#[should_panic(expected = "Call out of sequence")]
fn test_faked_call_out_of_order_panics() {
    let mock = seq_mock_mock();
    let spy = seq_plain_spy_spy();
    let mut seq = fnmock::Sequence::new_strict();
    mock.setup(|id| id);
    spy.expect(fnmock::predicate::eq(1))
        .once()
        .in_sequence(&mut seq);
    mock.expect(fnmock::predicate::eq(2))
        .once()
        .in_sequence(&mut seq);

    seq_mock(2); // the spy step must come first
}

#[test]
fn test_two_mocks_share_a_sequence() {
    let first = seq_mock_mock();
    let second = seq_other_mock_mock();
    let mut seq = fnmock::Sequence::new_strict();
    first
        .expect(fnmock::predicate::always())
        .once()
        .in_sequence(&mut seq);
    second
        .expect(fnmock::predicate::always())
        .once()
        .in_sequence(&mut seq);

    seq_mock(1);
    seq_other_mock(1);

    first.assert();
    second.assert();
}

#[test]
fn test_clear_removes_the_mocks_steps_from_a_sequence() {
    let mock = seq_mock_mock();
    let spy = seq_plain_spy_spy();
    let mut seq = fnmock::Sequence::new_strict();
    mock.expect(fnmock::predicate::always())
        .once()
        .in_sequence(&mut seq);
    spy.expect(fnmock::predicate::always())
        .once()
        .in_sequence(&mut seq);

    mock.clear();

    // The spy is now the first step; without the clear this is out of order.
    seq_plain_spy(1);

    spy.assert();
    mock.assert();
}
