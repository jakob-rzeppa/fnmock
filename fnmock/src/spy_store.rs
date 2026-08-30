//! Storage backing the spy of a single function.
//!
//! This is a fnmock internal. You should not interact with it directly.

use crate::{Sequence, call_range::CallRange, expectation::Expectation, matcher::Matcher};

pub struct SpyStore<M: Matcher> {
    /// How the spied function is named in panic messages.
    name: String,

    /// Holds all standalone expectations.
    expectations: Vec<Expectation<M>>,
    /// References to the Sequences associated with this function.
    /// Calls are passed through to the sequences.
    sequences: Vec<Sequence>,

    total_calls: usize,
    total_call_range: Option<CallRange>,
}

impl<M: Matcher + 'static> SpyStore<M> {
    /// Create a store with no expectations set.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            expectations: Vec::new(),
            sequences: Vec::new(),
            total_calls: 0,
            total_call_range: None,
        }
    }

    /// Expect this many calls of the spied function, whatever their arguments.
    ///
    /// This is independent of the expectations set with [`SpyStore::set_expectation`] and of
    /// any sequence.
    pub fn set_total_call_range(&mut self, call_range: CallRange) {
        self.total_call_range = Some(call_range);
    }

    /// Record a call of the spied function against every expectation set on it.
    ///
    /// Expectations outside a sequence each see the call on their own. Sequenced ones only see
    /// it through their sequence, which decides whether the call is the one it is waiting for.
    /// A call no expectation matches is not an error — the spied function still runs, and a
    /// spy only reports on the expectations a test actually set.
    ///
    /// # Panics
    ///
    /// Panics if the call exceeds the maximum of an expectation or of the total call range, or
    /// if it comes out of order in one of the sequences this function takes part in.
    pub fn record_call(&mut self, params: &M::Params<'_>) {
        self.total_calls += 1;
        if let Some(call_range) = &self.total_call_range {
            assert!(
                !call_range.max_exceeded(&self.total_calls),
                "Too many calls of the spied function '{}': got {}, expected {}.",
                self.name,
                self.total_calls,
                call_range
            );
        }

        // Standalone expectations
        for expectation in &mut self.expectations {
            expectation.call(params);
        }

        // Sequences
        for sequence in &self.sequences {
            sequence.record_call::<M>(params);
        }
    }

    /// The name the spied function is reported under, e.g. `get_user` or, for one instantiation
    /// of a generic function, `identity::<alloc::string::String>`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Check that every expectation set on the spied function is fulfilled.
    /// One message per expectation of this store that is not fulfilled, empty when it is
    /// satisfied.
    ///
    /// Split out of [`SpyStore::assert`] so that
    /// [`GenericSpyStore`](crate::generic_spy_store::GenericSpyStore) can collect the failures of
    /// every instantiation into a single panic instead of stopping at the first one.
    pub fn check_for_failures(&self) -> Vec<String> {
        let mut failures: Vec<String> = Vec::new();

        if let Some(call_range) = &self.total_call_range {
            if !call_range.contains(&self.total_calls) {
                failures.push(format!(
                    "the spied function '{}' was called {} time(s), expected {}",
                    self.name, self.total_calls, call_range
                ));
            }
        }

        for expectation in &self.expectations {
            if !expectation.is_fulfilled() {
                failures.push(format!(
                    "{} got {} matching call(s), expected {}",
                    expectation,
                    expectation.call_count(),
                    expectation.call_range()
                ));
            }
        }

        for sequence in &self.sequences {
            for (description, call_count, call_range) in sequence.unfulfilled_steps::<M>() {
                failures.push(format!(
                    "sequenced {} got {} matching call(s), expected {}",
                    description, call_count, call_range
                ));
            }
        }

        failures
    }

    /// Assert that every expectation set on the spied function is fulfilled.
    ///
    /// # Panics
    ///
    /// Panics if any expectation is not fulfilled.
    pub fn assert(&self) {
        let failures = self.check_for_failures();

        assert!(
            failures.is_empty(),
            "Expectation(s) of the spied function '{}' failed:\n{}",
            self.name,
            failures.join("\n")
        );
    }

    /// Add a standalone expectation
    pub fn add_expectation(&mut self, expectation: Expectation<M>) {
        self.expectations.push(expectation);
    }

    /// Add a sequence, skip if it already exists
    pub fn add_sequences(&mut self, sequences: Vec<Sequence>) {
        for sequence in sequences {
            if self
                .sequences
                .iter()
                .find(|e| e.id() == sequence.id())
                .is_none()
            {
                self.sequences.push(sequence);
            }
        }
    }
}
