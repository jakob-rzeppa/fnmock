//! One test per line of `docs/EXPECTATIONS.md`, in the order the document introduces them.
//!
//! Each test gets its own thread from the test harness, and the spy's store is thread local,
//! so the expectations set here never leak into another test.

use fnmock::{Sequence, predicate};

use super::get_user::{get_user, get_user_spy};

/// Call `get_user` with an `id` and a fixed `uuid`, so a test only varies one argument.
fn call(id: &str) {
    get_user(id.to_string(), "uuid");
}

mod matching {
    use super::*;

    #[test]
    fn expect_accepts_a_call_matching_every_predicate() {
        let spy = get_user_spy();
        spy.expect(predicate::eq("a".to_string()), predicate::eq("uuid"));

        call("a");

        spy.assert();
    }

    #[test]
    #[should_panic(expected = "Expectation(s) of the spied function")]
    fn expect_is_unfulfilled_when_no_call_matches() {
        let spy = get_user_spy();
        spy.expect(predicate::eq("a".to_string()), predicate::eq("other"));

        call("a");

        spy.assert();
    }

    #[test]
    fn expectf_matches_all_arguments_with_one_function() {
        let spy = get_user_spy();
        spy.expectf(|id, uuid| id == "a" && uuid == "uuid");

        call("a");

        spy.assert();
    }
}

mod times {
    use super::*;

    #[test]
    fn times_with_a_count_wants_exactly_that_many_calls() {
        let spy = get_user_spy();
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .times(3);

        call("a");
        call("a");
        call("a");

        spy.assert();
    }

    #[test]
    #[should_panic(expected = "Expectation(s) of the spied function")]
    fn times_with_a_count_is_unfulfilled_below_it() {
        let spy = get_user_spy();
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .times(3);

        call("a");
        call("a");

        spy.assert();
    }

    #[test]
    #[should_panic(expected = "Too many calls of the spied function")]
    fn a_call_past_the_count_panics_right_away() {
        let spy = get_user_spy();
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .times(3);

        call("a");
        call("a");
        call("a");
        call("a");
    }

    #[test]
    fn times_with_an_inclusive_range_accepts_any_count_inside_it() {
        let spy = get_user_spy();
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .times(1..=3);

        call("a");
        call("a");

        spy.assert();
    }

    #[test]
    fn times_with_an_open_end_accepts_any_count_from_its_start() {
        let spy = get_user_spy();
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .times(1..);

        call("a");
        call("a");
        call("a");
        call("a");

        spy.assert();
    }

    #[test]
    fn times_with_an_open_start_is_fulfilled_without_any_call() {
        let spy = get_user_spy();
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .times(..3);

        spy.assert();
    }

    #[test]
    #[should_panic(expected = "Too many calls of the spied function")]
    fn times_with_an_open_start_still_has_a_maximum() {
        let spy = get_user_spy();
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .times(..3);

        call("a");
        call("a");
        call("a");
    }

    #[test]
    fn once_wants_a_single_call() {
        let spy = get_user_spy();
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .once();

        call("a");

        spy.assert();
    }

    #[test]
    fn never_is_fulfilled_by_calls_that_do_not_match() {
        let spy = get_user_spy();
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .never();

        call("b");

        spy.assert();
    }

    #[test]
    #[should_panic(expected = "Too many calls of the spied function")]
    fn never_panics_on_a_matching_call() {
        let spy = get_user_spy();
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .never();

        call("a");
    }
}

mod global_times {
    use super::*;

    #[test]
    fn expect_times_counts_calls_whatever_their_arguments() {
        let spy = get_user_spy();
        spy.expect_times(2);

        call("a");
        call("b");

        spy.assert();
    }

    #[test]
    #[should_panic(expected = "Expectation(s) of the spied function")]
    fn expect_times_is_unfulfilled_below_its_count() {
        let spy = get_user_spy();
        spy.expect_times(2);

        call("a");

        spy.assert();
    }

    #[test]
    #[should_panic(expected = "Too many calls of the spied function")]
    fn a_call_past_expect_times_panics_right_away() {
        let spy = get_user_spy();
        spy.expect_times(2);

        call("a");
        call("b");
        call("c");
    }

    #[test]
    fn expect_once_wants_a_single_call() {
        let spy = get_user_spy();
        spy.expect_once();

        call("a");

        spy.assert();
    }

    #[test]
    #[should_panic(expected = "Too many calls of the spied function")]
    fn expect_never_panics_on_any_call() {
        let spy = get_user_spy();
        spy.expect_never();

        call("a");
    }

    #[test]
    fn expect_times_takes_a_range_too() {
        let spy = get_user_spy();
        spy.expect_times(2..);

        call("a");
        call("b");
        call("c");

        spy.assert();
    }

    #[test]
    fn expect_times_is_independent_of_the_argument_expectations() {
        let spy = get_user_spy();
        spy.expect_times(3);
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .once();

        call("a");
        call("b");
        call("c");

        spy.assert();
    }
}

mod no_sequence {
    use super::*;

    #[test]
    fn expectations_without_a_sequence_are_fulfilled_independently() {
        let spy = get_user_spy();
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .times(3);
        spy.expect(predicate::eq("b".to_string()), predicate::always())
            .times(1..=3);

        // Their order does not matter, only their counts.
        call("b");
        call("a");
        call("a");
        call("a");

        spy.assert();
    }

    #[test]
    #[should_panic(expected = "Expectation(s) of the spied function")]
    fn one_unfulfilled_expectation_fails_the_assert() {
        let spy = get_user_spy();
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .times(3);
        spy.expect(predicate::eq("b".to_string()), predicate::always())
            .times(1..=3);

        call("a");
        call("a");
        call("a");

        spy.assert();
    }
}

mod sequences {
    use super::*;

    #[test]
    fn calls_made_in_the_expected_order_fulfill_the_sequence() {
        let spy = get_user_spy();
        let mut seq = Sequence::new();
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .times(3)
            .in_sequence(&mut seq);
        spy.expect(predicate::eq("b".to_string()), predicate::always())
            .once()
            .in_sequence(&mut seq);

        call("a");
        call("a");
        call("a");
        call("b");

        spy.assert();
    }

    #[test]
    #[should_panic(expected = "Expectation(s) of the spied function")]
    fn a_call_reaching_a_later_step_too_early_is_ignored() {
        let spy = get_user_spy();
        let mut seq = Sequence::new();
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .times(3)
            .in_sequence(&mut seq);
        spy.expect(predicate::eq("b".to_string()), predicate::always())
            .once()
            .in_sequence(&mut seq);

        call("a");
        // Too early for the second step, so the sequence treats it like any other call it has
        // no use for: not recorded, and the sequence stays on the first step.
        call("b");
        call("a");
        call("a");

        // The early "b" was dropped rather than recorded, so the second step never got a call
        // and the order is caught here instead of at the call.
        spy.assert();
    }

    #[test]
    #[should_panic(expected = "Call out of sequence")]
    fn a_call_reaching_a_later_step_too_early_panics_in_a_strict_sequence() {
        let spy = get_user_spy();
        let mut seq = Sequence::new_strict();
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .times(3)
            .in_sequence(&mut seq);
        spy.expect(predicate::eq("b".to_string()), predicate::always())
            .once()
            .in_sequence(&mut seq);

        call("a");
        call("b");
    }

    #[test]
    fn a_strict_sequence_accepts_the_calls_made_in_order() {
        let spy = get_user_spy();
        let mut seq = Sequence::new_strict();
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .times(2)
            .in_sequence(&mut seq);
        spy.expect(predicate::eq("b".to_string()), predicate::always())
            .once()
            .in_sequence(&mut seq);
        // Strictness is about the order of the sequence's own steps, not about calls it has
        // nothing to do with.
        spy.expect(predicate::eq("z".to_string()), predicate::always())
            .once();

        call("a");
        call("z");
        call("a");
        call("stray");
        call("b");

        spy.assert();
    }

    #[test]
    #[should_panic(expected = "Expectation(s) of the spied function")]
    fn a_step_the_sequence_never_reached_fails_the_assert() {
        let spy = get_user_spy();
        let mut seq = Sequence::new();
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .once()
            .in_sequence(&mut seq);
        spy.expect(predicate::eq("b".to_string()), predicate::always())
            .once()
            .in_sequence(&mut seq);

        call("a");

        spy.assert();
    }

    /// The walkthrough from `docs/EXPECTATIONS.md`, step by step.
    #[test]
    fn every_kind_of_step_in_one_sequence() {
        let spy = get_user_spy();
        let mut seq = Sequence::new();
        // Advancable at any time, no minimum and no maximum.
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .times(..)
            .in_sequence(&mut seq);
        // One call advances the sequence.
        spy.expect(predicate::eq("b".to_string()), predicate::always())
            .once()
            .in_sequence(&mut seq);
        // Before the next step there can be no call with "c".
        spy.expect(predicate::eq("c".to_string()), predicate::always())
            .never()
            .in_sequence(&mut seq);
        // Advancable after one call, four or more panic.
        spy.expect(predicate::eq("d".to_string()), predicate::always())
            .times(1..4)
            .in_sequence(&mut seq);
        // Advancable after two calls, no maximum.
        spy.expect(predicate::eq("e".to_string()), predicate::always())
            .times(2..)
            .in_sequence(&mut seq);

        call("b");
        call("d");
        call("d");
        call("e");
        call("e");

        spy.assert();
    }

    #[test]
    #[should_panic(expected = "Too many calls of the spied function")]
    fn a_never_step_panics_while_it_is_current() {
        let spy = get_user_spy();
        let mut seq = Sequence::new();
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .once()
            .in_sequence(&mut seq);
        spy.expect(predicate::eq("b".to_string()), predicate::always())
            .never()
            .in_sequence(&mut seq);

        call("a");
        call("b");
    }

    #[test]
    fn a_greedy_step_starves_a_later_one_of_its_calls() {
        let spy = get_user_spy();
        let mut seq = Sequence::new();
        spy.expect(predicate::always(), predicate::always())
            .times(2)
            .in_sequence(&mut seq);
        spy.expect(predicate::eq("b".to_string()), predicate::always())
            .times(..)
            .in_sequence(&mut seq);

        // Both calls go to the first step, even though the second one would accept "b" too.
        call("b");
        call("b");

        spy.assert();
    }

    #[test]
    fn a_call_no_expectation_matches_is_not_an_error() {
        let spy = get_user_spy();
        let mut seq = Sequence::new();
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .once()
            .in_sequence(&mut seq);
        spy.expect(predicate::eq("b".to_string()), predicate::always())
            .once()
            .in_sequence(&mut seq);

        call("a");
        // Nothing expects this one. A spy does not replace the function, so it has no reason
        // to reject a call the test simply did not describe.
        call("stray");
        call("b");

        spy.assert();
    }

    #[test]
    fn two_sequences_advance_independently() {
        let spy = get_user_spy();
        let mut first = Sequence::new();
        let mut second = Sequence::new();
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .once()
            .in_sequence(&mut first);
        spy.expect(predicate::eq("b".to_string()), predicate::always())
            .once()
            .in_sequence(&mut first);
        spy.expect(predicate::eq("x".to_string()), predicate::always())
            .once()
            .in_sequence(&mut second);
        spy.expect(predicate::eq("y".to_string()), predicate::always())
            .once()
            .in_sequence(&mut second);

        // Interleaved: each sequence only sees the calls one of its steps accepts.
        call("x");
        call("a");
        call("y");
        call("b");

        spy.assert();
    }

    #[test]
    fn in_sequence_may_be_chained_before_times() {
        let spy = get_user_spy();
        let mut seq = Sequence::new();
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .in_sequence(&mut seq)
            .times(2);
        spy.expect(predicate::eq("b".to_string()), predicate::always())
            .in_sequence(&mut seq)
            .once();

        call("a");
        call("a");
        call("b");

        spy.assert();
    }

    #[test]
    #[should_panic(expected = "Call out of sequence")]
    fn a_range_set_after_in_sequence_still_holds_the_sequence_back() {
        let spy = get_user_spy();
        let mut seq = Sequence::new_strict();
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .in_sequence(&mut seq)
            .times(2);
        spy.expect(predicate::eq("b".to_string()), predicate::always())
            .in_sequence(&mut seq)
            .once();

        call("a");
        call("b");
    }

    #[test]
    fn an_unsequenced_expectation_lives_next_to_the_sequence() {
        let spy = get_user_spy();
        let mut seq = Sequence::new();
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .once()
            .in_sequence(&mut seq);
        spy.expect(predicate::eq("b".to_string()), predicate::always())
            .once()
            .in_sequence(&mut seq);
        // Not a step, so it is not ordered: its calls may come whenever, and they are none of
        // the sequence's business.
        spy.expect(predicate::eq("z".to_string()), predicate::always())
            .times(2);

        call("z");
        call("a");
        call("z");
        call("b");

        spy.assert();
    }

    #[test]
    fn a_call_matching_an_unsequenced_expectation_does_not_advance_the_sequence() {
        let spy = get_user_spy();
        let mut seq = Sequence::new();
        spy.expect(predicate::eq("a".to_string()), predicate::always())
            .times(2)
            .in_sequence(&mut seq);
        spy.expect(predicate::eq("b".to_string()), predicate::always())
            .once()
            .in_sequence(&mut seq);
        spy.expect(predicate::eq("z".to_string()), predicate::always())
            .once();

        call("a");
        // The first step is not advancable yet, but "z" is not out of order — it was never
        // part of the order.
        call("z");
        call("a");
        call("b");

        spy.assert();
    }
}
