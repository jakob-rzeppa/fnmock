//! An order calls have to be made in, possibly spanning several spied functions.

use std::{cell::RefCell, rc::Rc};

use crate::{
    call_range::CallRange,
    expectation::{DynExpectation, Expectation},
    matcher::Matcher,
};

/// An order the calls matching its expectations have to be made in.
///
/// A sequence is created by the test and handed to as many expectations as it should order.
/// Those expectations may belong to different spied functions, which is the point: a sequence
/// is the only way to say that one function has to be called before another.
///
/// ```ignore
/// let seq = Sequence::new();
/// get_user_spy().expect(eq(2)).times(3).in_sequence(&seq);
/// save_user_spy().expect(eq(2)).once().in_sequence(&seq);
/// ```
///
/// Cloning a sequence — which `in_sequence` does — shares it, so every spy taking part in it
/// sees the same progress. A test may use as many independent sequences as it likes.
///
/// Matching is **greedy**: the current step consumes every call it accepts, and the sequence
/// only moves on once a call arrives that a later step accepts instead. Moving on requires
/// every step in between to be *advancable*, meaning the minimum of its call range is reached.
///
/// A call the sequence cannot place — one no step accepts, or one that would need it to move
/// past a step that is not advancable yet — is none of this sequence's business and leaves it
/// untouched. That is what lets a sequence ignore the calls of a function it does not order,
/// the calls meant for another sequence, and the calls of expectations set outside any
/// sequence. The order is still enforced, just at the end: a step that was passed over never
/// got its calls, so the assert fails.
///
/// A [`Sequence::strict`] sequence does not let the second case pass. A call matching a later
/// step while an earlier one is unfinished panics right there, which pins down calls in the
/// wrong order more reliably.
#[derive(Clone)]
pub struct Sequence(Rc<RefCell<SequenceState>>);

struct SequenceState {
    steps: Vec<Box<dyn DynExpectation>>,
    current_step: usize,
    strict: bool,
}

impl Sequence {
    /// Create a sequence without any steps.
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(SequenceState {
            steps: Vec::new(),
            current_step: 0,
            strict: false,
        })))
    }

    /// Make this sequence panic on a call that matches a later step while an earlier one has
    /// not reached the minimum of its call range yet.
    pub fn new_strict() -> Self {
        Self(Rc::new(RefCell::new(SequenceState {
            steps: Vec::new(),
            current_step: 0,
            strict: true,
        })))
    }

    /// Identity of the shared state, so a spy can tell two clones of one sequence apart from
    /// two different sequences.
    pub fn id(&self) -> *const () {
        Rc::as_ptr(&self.0).cast::<()>()
    }

    pub fn append_expectation<M: Matcher>(&mut self, expectation: Expectation<M>) {
        self.0.borrow_mut().steps.push(Box::new(expectation));
    }

    /// Offer one call of a spied function to the sequence.
    ///
    /// # Panics
    ///
    /// A [`Sequence::strict`] sequence panics if the call belongs to a later step while an
    /// earlier one has not reached the minimum of its call range yet.
    pub fn record_call<M: Matcher>(&self, params: &M::Params<'_>) {
        let mut state = self.0.borrow_mut();

        // Greedy: the earliest step from the current one on that accepts the call gets it, even
        // if a later, more specific step would have accepted it too.
        // Position is short-circuiting, so it stops after the first match.
        let Some(offset) = state.steps[state.current_step..]
            .iter()
            .position(|expectation| {
                // In a sequence may be Expectations for different functions / matchers.
                // If downcast_ref returns None the expectation is for a different function.
                if let Some(expectation) = expectation.as_any().downcast_ref::<Expectation<M>>() {
                    expectation.matches(&params)
                } else {
                    false
                }
            })
        else {
            return;
        };

        let matched_step = state.current_step + offset;

        let blocked_steps: Vec<usize> = state.steps[state.current_step..matched_step]
            .iter()
            .enumerate()
            .filter_map(|(i, step)| {
                if !step.is_advancable() {
                    Some(state.current_step + i)
                } else {
                    None
                }
            })
            .collect();

        if !blocked_steps.is_empty() {
            if state.strict {
                let details = blocked_steps
                    .iter()
                    .map(|&idx| {
                        let step = &state.steps[idx];
                        format!(
                            "step {}: got {} calls, expected {}",
                            idx,
                            step.call_count(),
                            step.call_range()
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("; ");
                panic!(
                    "Call out of sequence: matched step {}, but {} blocking ({})",
                    matched_step,
                    if blocked_steps.len() == 1 {
                        "step is".to_string()
                    } else {
                        format!("{} steps are", blocked_steps.len())
                    },
                    details
                );
            }
            // Not a call this sequence can take yet, so it is treated like any other call that
            // does not apply to the current step.
            return;
        }

        state.current_step = matched_step;
        state.steps[matched_step].record_match();
    }

    /// Every step belonging to matcher type `M` that has not reached its call range, as
    /// `(display_str, call_count, call_range)`.
    ///
    /// Steps of other functions taking part in this sequence are silently skipped, the same
    /// way [`Sequence::record_call`] only ever matches its own function's steps.
    pub fn unfulfilled_steps<M: Matcher>(&self) -> Vec<(String, usize, CallRange)> {
        self.0
            .borrow()
            .steps
            .iter()
            .filter_map(|step| step.as_any().downcast_ref::<Expectation<M>>())
            .filter(|expectation| !expectation.is_fulfilled())
            .map(|expectation| {
                (
                    expectation.to_string(),
                    expectation.call_count(),
                    expectation.call_range(),
                )
            })
            .collect()
    }
}
