//! One sequence ordering the calls of two different spied functions.
//!
//! This is what a sequence exists for that a per-function order could not express: "`get_user`
//! has to run before `save_user`". Each spy keeps its own store and its own parameter type;
//! the shared sequence is the only thing they have in common.

use fnmock::{Sequence, predicate};

use super::{
    get_user::{get_user, get_user_spy},
    save_user::{save_user, save_user_spy},
};

#[test]
fn calls_of_two_functions_in_the_expected_order() {
    let get = get_user_spy();
    let save = save_user_spy();
    let mut seq = Sequence::new();
    get.expect(predicate::eq("a".to_string()), predicate::always())
        .once()
        .in_sequence(&mut seq);
    save.expect(predicate::eq("a".to_string()))
        .once()
        .in_sequence(&mut seq);

    //get_user("a".to_string(), "uuid");
    save_user("a".to_string());

    get.assert();
    save.assert();
}

/// Without `strict` the early call is simply ignored, and the test only fails at the assert.
#[test]
#[should_panic(expected = "Expectation(s) of the spied function")]
fn the_second_function_called_first_is_ignored() {
    let get = get_user_spy();
    let save = save_user_spy();
    let mut seq = Sequence::new();
    get.expect(predicate::eq("a".to_string()), predicate::always())
        .once()
        .in_sequence(&mut seq);
    save.expect(predicate::eq("a".to_string()))
        .once()
        .in_sequence(&mut seq);

    save_user("a".to_string());
    get_user("a".to_string(), "uuid");

    get.assert();
    save.assert();
}

#[test]
#[should_panic(expected = "Call out of sequence")]
fn the_second_function_may_not_be_called_first() {
    let get = get_user_spy();
    let save = save_user_spy();
    let mut seq = Sequence::new_strict();
    get.expect(predicate::eq("a".to_string()), predicate::always())
        .once()
        .in_sequence(&mut seq);
    save.expect(predicate::eq("a".to_string()))
        .once()
        .in_sequence(&mut seq);

    save_user("a".to_string());
}

#[test]
fn a_step_of_the_other_function_does_not_swallow_this_function_s_calls() {
    let get = get_user_spy();
    let save = save_user_spy();
    let mut seq = Sequence::new();
    get.expect(predicate::eq("a".to_string()), predicate::always())
        .times(2)
        .in_sequence(&mut seq);
    save.expect(predicate::eq("a".to_string()))
        .once()
        .in_sequence(&mut seq);
    // Not ordered against anything, and belonging to neither step.
    get.expect(predicate::eq("z".to_string()), predicate::always())
        .once();

    get_user("a".to_string(), "uuid");
    get_user("z".to_string(), "uuid");
    get_user("a".to_string(), "uuid");
    save_user("a".to_string());

    get.assert();
    save.assert();
}

#[test]
fn the_functions_may_alternate_across_several_steps() {
    let get = get_user_spy();
    let save = save_user_spy();
    let mut seq = Sequence::new();
    get.expect(predicate::eq("a".to_string()), predicate::always())
        .once()
        .in_sequence(&mut seq);
    save.expect(predicate::eq("a".to_string()))
        .once()
        .in_sequence(&mut seq);
    get.expect(predicate::eq("b".to_string()), predicate::always())
        .once()
        .in_sequence(&mut seq);
    save.expect(predicate::eq("b".to_string()))
        .once()
        .in_sequence(&mut seq);

    get_user("a".to_string(), "uuid");
    save_user("a".to_string());
    get_user("b".to_string(), "uuid");
    save_user("b".to_string());

    get.assert();
    save.assert();
}

#[test]
#[should_panic(expected = "Call out of sequence")]
fn a_step_of_the_other_function_still_blocks_a_later_step_of_this_one() {
    let get = get_user_spy();
    let save = save_user_spy();
    let mut seq = Sequence::new_strict();
    get.expect(predicate::eq("a".to_string()), predicate::always())
        .once()
        .in_sequence(&mut seq);
    save.expect(predicate::eq("a".to_string()))
        .once()
        .in_sequence(&mut seq);
    get.expect(predicate::eq("b".to_string()), predicate::always())
        .once()
        .in_sequence(&mut seq);

    get_user("a".to_string(), "uuid");
    // `save_user` never ran, so its step is not advancable and "b" comes too early.
    get_user("b".to_string(), "uuid");
}
