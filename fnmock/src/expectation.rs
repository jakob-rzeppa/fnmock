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
    /// The name is used to display the expectation in case it wasn't fulfilled.
    name: Option<String>,

    /// The spied function this expectation belongs to, so panics can name it even when the
    /// caller (e.g. a [`Sequence`](crate::sequence::Sequence) spanning several functions) has
    /// no other way to know.
    function_name: &'static str,

    matcher: M,

    call_range: CallRange,
    call_count: usize,
}

impl<M: Matcher> Expectation<M> {
    /// Create an expectation matching `matcher` on the spied function `function_name`,
    /// expecting at least one call.
    pub fn new(matcher: M, function_name: &'static str) -> Self {
        Self {
            name: None,
            function_name,
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
            "Too many calls of the spied function '{}': expectation '{}' got {}, expected {}.",
            self.function_name,
            self,
            self.call_count,
            self.call_range
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

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Name this expectation together with the spied function it belongs to, so a failure
    /// naming it never depends on surrounding context (a header, a log line above it, ...) to
    /// say which function it is about.
    pub fn describe(&self) -> String {
        format!(
            "expectation '{}' of the spied function '{}'",
            self, self.function_name
        )
    }
}

impl<M: Matcher> Display for Expectation<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref name) = self.name {
            write!(f, "{}", name)?;
        } else {
            // Fallback to matcher
            write!(f, "{}", &self.matcher)?;
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
    /// Name this expectation together with the spied function it belongs to, for panics raised
    /// by a caller (e.g. [`Sequence`](crate::sequence::Sequence)) that only sees the dyn-safe
    /// view and so has no other way to name the function.
    fn describe(&self) -> String;
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

    fn describe(&self) -> String {
        Expectation::describe(self)
    }
}
