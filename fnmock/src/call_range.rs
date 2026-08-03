use std::{
    fmt::Display,
    ops::{Bound, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
};

/// A call-count range, converted from any concrete `RangeBounds<usize>` impl.
///
/// `Expectation` can't stay generic over the range type it was built with,
/// since that would give every concrete range (`Range<usize>`,
/// `RangeInclusive<usize>`, ...) its own `Expectation<Range>` type, and a
/// `Vec` can only hold one concrete type. Converting into `CallRange` up
/// front erases that difference so expectations built from different range
/// literals can live in the same `Vec`.
///
/// The bounds are stored as `Bound<usize>` rather than `Range<usize>` so that
/// an unbounded end (`5..`, `..`) stays truly unbounded instead of being
/// approximated with a `usize::MAX` sentinel.
#[derive(Clone, Copy, Debug)]
pub struct CallRange {
    start: Bound<usize>,
    end: Bound<usize>,
}

impl CallRange {
    pub fn contains(&self, value: &usize) -> bool {
        self.min_reached(value) && !self.max_exceeded(value)
    }

    /// Whether `value` reached the start of the range, ignoring its end.
    ///
    /// This is what makes an expectation inside a sequence advancable: the minimum has to be
    /// satisfied before the sequence moves on, while further calls may still be accepted.
    pub fn min_reached(&self, value: &usize) -> bool {
        match self.start {
            Bound::Included(start) => *value >= start,
            Bound::Excluded(start) => *value > start,
            Bound::Unbounded => true,
        }
    }

    pub fn max_exceeded(&self, value: &usize) -> bool {
        match self.end {
            Bound::Unbounded => false,
            Bound::Included(max) => max < *value,
            Bound::Excluded(max) => max <= *value,
        }
    }
}

impl Display for CallRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.start, &self.end) {
            (Bound::Included(n), Bound::Included(m)) if n == m => {
                write!(f, "exactly {}", n)
            }
            (Bound::Included(n), Bound::Excluded(m)) => {
                write!(f, "{}..{}", n, m)
            }
            (Bound::Included(n), Bound::Included(m)) => {
                write!(f, "{}..={}", n, m)
            }
            (Bound::Included(n), Bound::Unbounded) => {
                write!(f, "{}..", n)
            }
            (Bound::Excluded(n), Bound::Excluded(m)) => {
                write!(f, "({})..{}", n, m)
            }
            (Bound::Excluded(n), Bound::Included(m)) => {
                write!(f, "({})..={}", n, m)
            }
            (Bound::Excluded(n), Bound::Unbounded) => {
                write!(f, "({})..", n)
            }
            (Bound::Unbounded, Bound::Excluded(m)) => {
                write!(f, "..{}", m)
            }
            (Bound::Unbounded, Bound::Included(m)) => {
                write!(f, "..={}", m)
            }
            (Bound::Unbounded, Bound::Unbounded) => {
                write!(f, "..")
            }
        }
    }
}

impl From<usize> for CallRange {
    fn from(n: usize) -> Self {
        Self {
            start: Bound::Included(n),
            end: Bound::Included(n),
        }
    }
}

impl From<Range<usize>> for CallRange {
    fn from(r: Range<usize>) -> Self {
        assert!(r.end > r.start, "backwards range");
        Self {
            start: Bound::Included(r.start),
            end: Bound::Excluded(r.end),
        }
    }
}

impl From<RangeFrom<usize>> for CallRange {
    fn from(r: RangeFrom<usize>) -> Self {
        Self {
            start: Bound::Included(r.start),
            end: Bound::Unbounded,
        }
    }
}

impl From<RangeFull> for CallRange {
    fn from(_: RangeFull) -> Self {
        Self {
            start: Bound::Unbounded,
            end: Bound::Unbounded,
        }
    }
}

impl From<RangeInclusive<usize>> for CallRange {
    fn from(r: RangeInclusive<usize>) -> Self {
        assert!(r.end() >= r.start(), "backwards range");
        Self {
            start: Bound::Included(*r.start()),
            end: Bound::Included(*r.end()),
        }
    }
}

impl From<RangeTo<usize>> for CallRange {
    fn from(r: RangeTo<usize>) -> Self {
        Self {
            start: Bound::Unbounded,
            end: Bound::Excluded(r.end),
        }
    }
}

impl From<RangeToInclusive<usize>> for CallRange {
    fn from(r: RangeToInclusive<usize>) -> Self {
        Self {
            start: Bound::Unbounded,
            end: Bound::Included(r.end),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usize_contains_only_that_value() {
        let range: CallRange = 5.into();

        assert!(!range.contains(&4));
        assert!(range.contains(&5));
        assert!(!range.contains(&6));
    }

    #[test]
    fn usize_after_end_once_value_exceeds_it() {
        let range: CallRange = 5.into();

        assert!(!range.max_exceeded(&5));
        assert!(range.max_exceeded(&6));
    }

    #[test]
    fn usize_max_is_representable_without_overflow() {
        let range: CallRange = usize::MAX.into();

        assert!(!range.contains(&(usize::MAX - 1)));
        assert!(range.contains(&usize::MAX));
        assert!(!range.max_exceeded(&usize::MAX));
    }

    #[test]
    fn range_contains_start_but_not_end() {
        let range: CallRange = (2..5).into();

        assert!(!range.contains(&1));
        assert!(range.contains(&2));
        assert!(range.contains(&4));
        assert!(!range.contains(&5));
    }

    #[test]
    fn range_is_after_end_at_and_past_the_exclusive_end() {
        let range: CallRange = (2..5).into();

        assert!(!range.max_exceeded(&4));
        assert!(range.max_exceeded(&5));
        assert!(range.max_exceeded(&6));
    }

    #[test]
    #[should_panic(expected = "backwards range")]
    #[allow(clippy::reversed_empty_ranges, reason = "the point of the test")]
    fn range_backwards_panics() {
        let _range: CallRange = (5..2).into();
    }

    #[test]
    #[should_panic(expected = "backwards range")]
    fn range_empty_panics() {
        let _range: CallRange = (3..3).into();
    }

    #[test]
    fn range_from_contains_start_and_beyond() {
        let range: CallRange = (5..).into();

        assert!(!range.contains(&4));
        assert!(range.contains(&5));
        assert!(range.contains(&usize::MAX));
    }

    #[test]
    fn range_from_is_never_greater() {
        let range: CallRange = (5..).into();

        assert!(!range.max_exceeded(&5));
        assert!(!range.max_exceeded(&usize::MAX));
    }

    #[test]
    fn range_full_contains_everything() {
        let range: CallRange = (..).into();

        assert!(range.contains(&0));
        assert!(range.contains(&usize::MAX));
    }

    #[test]
    fn range_full_is_never_after_end() {
        let range: CallRange = (..).into();

        assert!(!range.max_exceeded(&0));
        assert!(!range.max_exceeded(&usize::MAX));
    }

    #[test]
    fn range_inclusive_contains_both_ends() {
        let range: CallRange = (2..=5).into();

        assert!(!range.contains(&1));
        assert!(range.contains(&2));
        assert!(range.contains(&5));
        assert!(!range.contains(&6));
    }

    #[test]
    fn range_inclusive_after_end_only_past_the_end() {
        let range: CallRange = (2..=5).into();

        assert!(!range.max_exceeded(&5));
        assert!(range.max_exceeded(&6));
    }

    #[test]
    fn range_inclusive_allows_a_single_value() {
        let range: CallRange = (3..=3).into();

        assert!(!range.contains(&2));
        assert!(range.contains(&3));
        assert!(!range.contains(&4));
    }

    #[test]
    #[should_panic(expected = "backwards range")]
    #[allow(clippy::reversed_empty_ranges, reason = "the point of the test")]
    fn range_inclusive_backwards_panics() {
        let _range: CallRange = (5..=2).into();
    }

    #[test]
    fn range_inclusive_max_end_is_representable_without_overflow() {
        let range: CallRange = (0..=usize::MAX).into();

        assert!(range.contains(&usize::MAX));
        assert!(!range.max_exceeded(&usize::MAX));
    }

    #[test]
    fn range_to_excludes_end_and_includes_zero() {
        let range: CallRange = (..5).into();

        assert!(range.contains(&0));
        assert!(range.contains(&4));
        assert!(!range.contains(&5));
    }

    #[test]
    fn range_to_is_after_end_at_and_past_the_exclusive_end() {
        let range: CallRange = (..5).into();

        assert!(!range.max_exceeded(&4));
        assert!(range.max_exceeded(&5));
    }

    #[test]
    fn range_to_inclusive_includes_end() {
        let range: CallRange = (..=5).into();

        assert!(range.contains(&0));
        assert!(range.contains(&5));
        assert!(!range.contains(&6));
    }

    #[test]
    fn range_to_inclusive_is_after_end_only_past_the_end() {
        let range: CallRange = (..=5).into();

        assert!(!range.max_exceeded(&5));
        assert!(range.max_exceeded(&6));
    }

    #[test]
    fn range_to_inclusive_max_end_is_representable_without_overflow() {
        let range: CallRange = (..=usize::MAX).into();

        assert!(range.contains(&usize::MAX));
        assert!(!range.max_exceeded(&usize::MAX));
    }

    #[test]
    fn display_usize_shows_exactly() {
        let range: CallRange = 5.into();
        assert_eq!(range.to_string(), "exactly 5");
    }

    #[test]
    fn display_range_shows_exclusive_end() {
        let range: CallRange = (2..5).into();
        assert_eq!(range.to_string(), "2..5");
    }

    #[test]
    fn display_range_inclusive_shows_inclusive_end() {
        let range: CallRange = (2..=5).into();
        assert_eq!(range.to_string(), "2..=5");
    }

    #[test]
    fn display_range_from_shows_unbounded_end() {
        let range: CallRange = (5..).into();
        assert_eq!(range.to_string(), "5..");
    }

    #[test]
    fn display_range_to_shows_unbounded_start() {
        let range: CallRange = (..5).into();
        assert_eq!(range.to_string(), "..5");
    }

    #[test]
    fn display_range_to_inclusive_shows_unbounded_start_with_inclusive_end() {
        let range: CallRange = (..=5).into();
        assert_eq!(range.to_string(), "..=5");
    }

    #[test]
    fn display_range_full_shows_unbounded() {
        let range: CallRange = (..).into();
        assert_eq!(range.to_string(), "..");
    }
}
