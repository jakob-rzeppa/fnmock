//! Storage backing the spy of a single function.
//!
//! This is a fnmock internal. You should not interact with it directly.

use crate::spy::{
    call_range::CallRange, expectation::Expectation, matcher::Matcher, sequence::Sequence,
};

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

    pub fn clear(&mut self) {
        self.total_calls = 0;
        self.total_call_range = None;

        self.expectations.clear();

        // We need to remove the expectations from the sequences before clearing the array,
        // since the sequences are shared between different spies.
        self.sequences
            .iter_mut()
            .for_each(|seq| seq.clear_expectations_for::<M>());
        self.sequences.clear();
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use super::*;

    #[derive(Clone)]
    struct IntMatcher(i32);

    impl Display for IntMatcher {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "eq({})", self.0)
        }
    }

    impl Matcher for IntMatcher {
        type Params<'a> = (&'a i32,);

        fn matches(&self, params: &Self::Params<'_>) -> bool {
            params.0 == &self.0
        }
    }

    #[derive(Clone)]
    struct StrMatcher(&'static str);

    impl Display for StrMatcher {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "eq({:?})", self.0)
        }
    }

    impl Matcher for StrMatcher {
        type Params<'a> = (&'a str,);

        fn matches(&self, params: &Self::Params<'_>) -> bool {
            params.0 == self.0
        }
    }

    fn int_expectation(value: i32, range: impl Into<CallRange>) -> Expectation<IntMatcher> {
        let mut expectation = Expectation::new(IntMatcher(value), "f");
        expectation.set_call_range(range.into());
        expectation
    }

    fn call(store: &mut SpyStore<IntMatcher>, value: i32) {
        store.record_call(&(&value,));
    }

    #[test]
    fn name_returns_the_name_given_at_creation() {
        let store = SpyStore::<IntMatcher>::new("get_user");
        assert_eq!(store.name(), "get_user");
    }

    #[test]
    fn new_store_has_no_failures() {
        let store = SpyStore::<IntMatcher>::new("f");
        assert!(store.check_for_failures().is_empty());
        store.assert();
    }

    #[test]
    fn calls_without_any_expectation_are_not_an_error() {
        let mut store = SpyStore::<IntMatcher>::new("f");
        call(&mut store, 1);
        store.assert();
    }

    #[test]
    fn expectation_is_fulfilled_by_matching_calls() {
        let mut store = SpyStore::<IntMatcher>::new("f");
        store.add_expectation(int_expectation(1, 2));

        call(&mut store, 1);
        call(&mut store, 1);

        store.assert();
    }

    #[test]
    fn non_matching_calls_do_not_count_towards_an_expectation() {
        let mut store = SpyStore::<IntMatcher>::new("f");
        store.add_expectation(int_expectation(1, 1));

        call(&mut store, 2);

        let failures = store.check_for_failures();
        assert_eq!(failures.len(), 1);
        assert!(failures[0].contains("eq(1)"));
        assert!(failures[0].contains("got 0 matching call(s)"));
    }

    #[test]
    fn every_expectation_sees_every_matching_call() {
        let mut store = SpyStore::<IntMatcher>::new("f");
        store.add_expectation(int_expectation(1, 1));
        store.add_expectation(int_expectation(1, 1));

        call(&mut store, 1);

        store.assert();
    }

    #[test]
    fn each_unfulfilled_expectation_gets_its_own_failure() {
        let mut store = SpyStore::<IntMatcher>::new("f");
        store.add_expectation(int_expectation(1, 1));
        store.add_expectation(int_expectation(2, 1));

        assert_eq!(store.check_for_failures().len(), 2);
    }

    #[test]
    #[should_panic(expected = "Too many calls of the spied function")]
    fn exceeding_an_expectations_maximum_panics() {
        let mut store = SpyStore::<IntMatcher>::new("f");
        store.add_expectation(int_expectation(1, 1));

        call(&mut store, 1);
        call(&mut store, 1);
    }

    #[test]
    fn total_call_range_counts_calls_regardless_of_arguments() {
        let mut store = SpyStore::<IntMatcher>::new("f");
        store.set_total_call_range(3.into());

        call(&mut store, 1);
        call(&mut store, 2);
        call(&mut store, 3);

        store.assert();
    }

    #[test]
    fn total_call_range_below_its_minimum_fails() {
        let mut store = SpyStore::<IntMatcher>::new("f");
        store.set_total_call_range(3.into());

        call(&mut store, 1);

        let failures = store.check_for_failures();
        assert_eq!(failures.len(), 1);
        assert!(failures[0].contains("'f' was called 1 time(s)"));
    }

    #[test]
    #[should_panic(expected = "Too many calls of the spied function 'f': got 2, expected")]
    fn exceeding_the_total_call_range_panics() {
        let mut store = SpyStore::<IntMatcher>::new("f");
        store.set_total_call_range(1.into());

        call(&mut store, 1);
        call(&mut store, 2);
    }

    #[test]
    #[should_panic(expected = "Expectation(s) of the spied function 'f' failed")]
    fn assert_panics_naming_the_function_when_unfulfilled() {
        let mut store = SpyStore::<IntMatcher>::new("f");
        store.add_expectation(int_expectation(1, 1));

        store.assert();
    }

    #[test]
    fn add_sequences_skips_a_sequence_already_added() {
        let mut store = SpyStore::<IntMatcher>::new("f");
        let seq = Sequence::new();

        store.add_sequences(vec![seq.clone()]);
        store.add_sequences(vec![seq.clone(), seq]);

        assert_eq!(store.sequences.len(), 1);
    }

    #[test]
    fn add_sequences_keeps_different_sequences() {
        let mut store = SpyStore::<IntMatcher>::new("f");

        store.add_sequences(vec![Sequence::new(), Sequence::new()]);

        assert_eq!(store.sequences.len(), 2);
    }

    #[test]
    fn calls_are_passed_through_to_sequences() {
        let mut store = SpyStore::<IntMatcher>::new("f");
        let mut seq = Sequence::new();
        seq.append_expectation(int_expectation(1, 1));
        store.add_sequences(vec![seq]);

        call(&mut store, 1);

        store.assert();
    }

    #[test]
    fn unfulfilled_sequence_step_is_reported_as_sequenced() {
        let mut store = SpyStore::<IntMatcher>::new("f");
        let mut seq = Sequence::new();
        seq.append_expectation(int_expectation(1, 1));
        store.add_sequences(vec![seq]);

        let failures = store.check_for_failures();
        assert_eq!(failures.len(), 1);
        assert!(failures[0].starts_with("sequenced "));
    }

    #[test]
    #[should_panic(expected = "Call out of sequence")]
    fn strict_sequence_panic_propagates_from_record_call() {
        let mut store = SpyStore::<IntMatcher>::new("f");
        let mut seq = Sequence::new_strict();
        seq.append_expectation(int_expectation(1, 2..));
        seq.append_expectation(int_expectation(2, 1));
        store.add_sequences(vec![seq]);

        call(&mut store, 1);
        call(&mut store, 2);
    }

    #[test]
    fn clear_resets_expectations_and_call_counts() {
        let mut store = SpyStore::<IntMatcher>::new("f");
        store.set_total_call_range(1.into());
        store.add_expectation(int_expectation(1, 1));
        call(&mut store, 1);

        store.clear();

        // Would fail if the total range, expectation or call count survived.
        store.assert();
    }

    #[test]
    fn clear_removes_own_steps_from_shared_sequences_only() {
        let mut int_store = SpyStore::<IntMatcher>::new("f");
        let mut str_store = SpyStore::<StrMatcher>::new("g");
        let mut seq = Sequence::new();
        seq.append_expectation(int_expectation(1, 1));
        seq.append_expectation(Expectation::new(StrMatcher("bob"), "g"));
        int_store.add_sequences(vec![seq.clone()]);
        str_store.add_sequences(vec![seq]);

        int_store.clear();

        assert!(int_store.check_for_failures().is_empty());
        assert_eq!(str_store.check_for_failures().len(), 1);
    }

    #[test]
    fn clear_detaches_the_store_from_its_sequences() {
        let mut store = SpyStore::<IntMatcher>::new("f");
        store.add_sequences(vec![Sequence::new()]);

        store.clear();

        assert!(store.sequences.is_empty());
    }

    #[test]
    fn clear_keeps_the_name() {
        let mut store = SpyStore::<IntMatcher>::new("f");
        store.clear();
        assert_eq!(store.name(), "f");
    }
}
