//! A single expectation set on a spy, and the handle a test configures it through.

use std::{any::Any, fmt::Display};

use crate::{call_range::CallRange, matcher::Matcher};

/// One expectation set on a spy: which calls it accepts, and how many of them.
///
/// A call is counted only if the [`Matcher`] accepts its arguments; calls the matcher rejects
/// are ignored, so several expectations with different argument matchers can coexist on the
/// same function.
#[derive(Clone)]
pub struct Expectation<M: Matcher> {
    /// The name is used to display which expectation failed.
    display_name: Option<String>,

    matcher: M,

    call_range: CallRange,
    call_count: usize,
}

impl<M: Matcher> Expectation<M> {
    /// Create an expectation matching `matcher`, expecting at least one call.
    pub fn new(matcher: M) -> Self {
        Self {
            display_name: None,
            matcher,
            call_range: (1..).into(),
            call_count: 0,
        }
    }

    /// Whether this expectation accepts the arguments of a call.
    pub fn matches(&self, params: &M::Params<'_>) -> bool {
        self.matcher.matches(params)
    }

    /// Record a call against this expectation, if the matcher accepts its arguments.
    ///
    /// # Panics
    ///
    /// Panics if the call pushes the count past the end of the call range.
    pub fn call(&mut self, params: &M::Params<'_>) {
        if self.matches(params) {
            self.record_match();
        }
    }

    /// Count a call already known to match, without consulting the matcher again.
    ///
    /// # Panics
    ///
    /// Panics if the call pushes the count past the end of the call range.
    fn record_match(&mut self) {
        self.call_count += 1;

        assert!(
            !self.call_range.max_exceeded(&self.call_count),
            "Too many calls."
        );
    }

    /// Whether the number of matching calls so far lies within the call range.
    pub fn is_fulfilled(&self) -> bool {
        self.call_range.contains(&self.call_count)
    }

    /// The number of matching calls recorded so far.
    pub fn call_count(&self) -> usize {
        self.call_count
    }

    fn is_advancable(&self) -> bool {
        self.call_range.min_reached(&self.call_count)
    }

    pub fn call_range(&self) -> CallRange {
        self.call_range
    }

    pub fn set_call_range(&mut self, call_range: CallRange) {
        self.call_range = call_range;
    }

    pub fn set_display_name(&mut self, display_name: String) {
        self.display_name = Some(display_name);
    }
}

impl<M: Matcher> Display for Expectation<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref display_name) = self.display_name {
            write!(f, "{}", display_name);
        } else {
            // Fallback to matcher
            write!(f, "{}", &self.matcher);
        }
        Ok(())
    }
}

/// Dyn-safe view of an [`Expectation<M>`], for storing expectations of different `M` alongside
/// each other (e.g. the steps of a [`Sequence`](crate::sequence::Sequence)).
///
/// [`DynExpectation::as_any`] is the escape hatch back to the concrete type, needed wherever a
/// caller does know which `M` it is looking for (e.g. to check [`Expectation::matches`]).
pub trait DynExpectation: Any {
    /// See [`Expectation::call_count`].
    fn call_count(&self) -> usize;
    /// See [`Expectation::call_range`].
    fn call_range(&self) -> CallRange;
    /// See [`Expectation::is_advancable`].
    fn is_advancable(&self) -> bool;
    /// See [`Expectation::record_match`].
    fn record_match(&mut self);
    /// Borrow this expectation as [`Any`], to attempt a downcast to a concrete `Expectation<M>`.
    fn as_any(&self) -> &dyn Any;
}

impl<M: Matcher> DynExpectation for Expectation<M> {
    fn call_count(&self) -> usize {
        self.call_count()
    }

    fn call_range(&self) -> CallRange {
        self.call_range()
    }

    fn is_advancable(&self) -> bool {
        self.is_advancable()
    }

    fn record_match(&mut self) {
        self.record_match();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
