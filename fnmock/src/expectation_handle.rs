use crate::{Sequence, call_range::CallRange, expectation::Expectation, matcher::Matcher};

/// What `expect` hands back to the test, so the expectation can be refined by chaining.
///
/// Dropping the handle leaves the expectation in place with whatever was configured on it;
/// `spy.expect(..)` on its own is a complete expectation of "at least one matching call".
pub struct ExpectationHandle<M: Matcher + 'static> {
    /// We use a option, since we need to take Expectation<M> in drop out of the struct,
    /// but drop only gives us a &mut self
    expectation: Option<Expectation<M>>,
    sequences: Option<Vec<Sequence>>,

    /// The callbacks
    expectation_callback: Box<dyn Fn(Expectation<M>)>,
    sequence_callback: Box<dyn Fn(Vec<Sequence>)>,
}

impl<M: Matcher> ExpectationHandle<M> {
    /// Wrap the expectation the store just registered, together with the spy's set of
    /// sequences, so [`ExpectationHandle::in_sequence`] can add one the spy does not know yet.
    ///
    /// `function_name` is the spied function this expectation belongs to, so panics raised on
    /// it (directly, or through a [`Sequence`] spanning several functions) can always say which
    /// function they came from.
    pub fn new(
        matcher: M,
        function_name: impl Into<String>,
        expectation_callback: impl Fn(Expectation<M>) + 'static,
        sequence_callback: impl Fn(Vec<Sequence>) + 'static,
    ) -> Self {
        Self {
            expectation: Some(Expectation::new(matcher, function_name)),
            sequences: Some(Vec::new()),
            expectation_callback: Box::new(expectation_callback),
            sequence_callback: Box::new(sequence_callback),
        }
    }

    /// Expect this many matching calls — a count (`3`) or any range (`1..=3`, `2..`, `..3`).
    pub fn times(mut self, call_range: impl Into<CallRange>) -> Self {
        if let Some(ref mut expectation) = self.expectation {
            expectation.set_call_range(call_range.into());
        } else {
            unreachable!(
                "The expectation of ExpectationHandle is None in times. This cannot happen because the only place the expectation in taken is in drop and times cannot be called after drop."
            )
        }
        self
    }

    /// Expect exactly one matching call.
    pub fn once(self) -> Self {
        self.times(1)
    }

    /// Expect no matching call at all.
    pub fn never(self) -> Self {
        self.times(0)
    }

    /// Set the display name of the expectation displayed in error messages.
    pub fn describe(mut self, display_name: String) -> Self {
        if let Some(ref mut expectation) = self.expectation {
            expectation.set_name(display_name);
        } else {
            unreachable!(
                "The expectation of ExpectationHandle is None in display. This cannot happen because the only place the expectation in taken is in drop and display cannot be called after drop."
            )
        }
        self
    }

    /// Add a sequence the expectation is appended to in drop.
    pub fn in_sequence(mut self, sequence: &mut Sequence) -> Self {
        if let Some(ref mut sequences) = self.sequences {
            sequences.push(sequence.clone());
        } else {
            unreachable!(
                "The sequences field in ExpectationHandle is None in in_sequence. This cannot happen because the only place the sequences are taken is in drop and in_sequence cannot be called after drop."
            );
        }
        self
    }
}

impl<M: Matcher> Drop for ExpectationHandle<M> {
    /// Here we register the expectation or sequences to the store.
    fn drop(&mut self) {
        let Some(expectation) = self.expectation.take() else {
            unreachable!(
                "The expectation of ExpectationHandle is None in drop. This cannot happen because the only place the expectation in taken is in drop and drop cannot be called twice."
            )
        };

        let Some(mut sequences) = self.sequences.take() else {
            unreachable!(
                "The sequences field in ExpectationHandle is None in drop. This cannot happen because the only place the sequences are taken is in drop and drop cannot be called twice."
            )
        };

        if sequences.is_empty() {
            (self.expectation_callback)(expectation);
        } else {
            for sequence in sequences.iter_mut() {
                sequence.append_expectation(expectation.clone());
            }
            (self.sequence_callback)(sequences)
        }
    }
}
