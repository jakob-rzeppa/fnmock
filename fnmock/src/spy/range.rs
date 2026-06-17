use std::{ fmt::Display, ops::{ Bound, RangeBounds } };

pub struct CallRange {
    start: Bound<usize>,
    end: Bound<usize>,
}

impl CallRange {
    pub fn from_range<R: RangeBounds<usize>>(range: R) -> Self {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&n) => Bound::Included(n),
            std::ops::Bound::Excluded(&n) => Bound::Excluded(n),
            std::ops::Bound::Unbounded => Bound::Unbounded,
        };

        let end = match range.end_bound() {
            std::ops::Bound::Included(&n) => Bound::Included(n),
            std::ops::Bound::Excluded(&n) => Bound::Excluded(n),
            std::ops::Bound::Unbounded => Bound::Unbounded,
        };

        CallRange { start, end }
    }

    pub fn is_at_max(&self, value: usize) -> bool {
        match self.end {
            Bound::Included(n) => value == n,
            Bound::Excluded(n) => value == n - 1,
            Bound::Unbounded => false,
        }
    }

    pub fn is_within(&self, value: usize) -> bool {
        let start_ok = match self.start {
            Bound::Included(n) => value >= n,
            Bound::Excluded(n) => value > n,
            Bound::Unbounded => true,
        };

        let end_ok = match self.end {
            Bound::Included(n) => value <= n,
            Bound::Excluded(n) => value < n,
            Bound::Unbounded => true,
        };

        start_ok && end_ok
    }
}

impl Display for CallRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.start, self.end) {
            (Bound::Included(start), Bound::Included(end)) => {
                if start == end {
                    write!(f, "exactly {} calls", start)
                } else {
                    write!(f, "in {}..={} calls", start, end)
                }
            }
            (Bound::Included(start), Bound::Excluded(end)) =>
                write!(f, "in {}..{} calls", start, end),
            (Bound::Excluded(start), Bound::Included(end)) =>
                write!(f, "in {}..={} calls", start + 1, end),
            (Bound::Excluded(start), Bound::Excluded(end)) =>
                write!(f, "in {}..{} calls", start + 1, end),
            (Bound::Included(start), Bound::Unbounded) => write!(f, "at least {} calls", start),
            (Bound::Excluded(start), Bound::Unbounded) => write!(f, "more than {} calls", start),
            (Bound::Unbounded, Bound::Unbounded) => write!(f, "any number of calls"),
            (Bound::Unbounded, Bound::Included(end)) => write!(f, "at most {} calls", end),
            (Bound::Unbounded, Bound::Excluded(end)) => write!(f, "less than {} calls", end),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_within_included_included() {
        let range = CallRange { start: Bound::Included(2), end: Bound::Included(4) };
        assert!(!range.is_within(1));
        assert!(range.is_within(2));
        assert!(range.is_within(3));
        assert!(range.is_within(4));
        assert!(!range.is_within(1));
        assert!(!range.is_within(5));
    }

    #[test]
    fn test_is_within_excluded_included() {
        let range = CallRange { start: Bound::Excluded(2), end: Bound::Included(4) };
        assert!(!range.is_within(1));
        assert!(!range.is_within(2));
        assert!(range.is_within(3));
        assert!(range.is_within(4));
        assert!(!range.is_within(5));
    }

    #[test]
    fn test_is_within_included_excluded() {
        let range = CallRange { start: Bound::Included(2), end: Bound::Excluded(4) };
        assert!(!range.is_within(1));
        assert!(range.is_within(2));
        assert!(range.is_within(3));
        assert!(!range.is_within(4));
        assert!(!range.is_within(5));
    }

    #[test]
    fn test_is_within_excluded_excluded() {
        let range = CallRange { start: Bound::Excluded(2), end: Bound::Excluded(4) };
        assert!(!range.is_within(1));
        assert!(!range.is_within(2));
        assert!(range.is_within(3));
        assert!(!range.is_within(4));
        assert!(!range.is_within(5));
    }

    #[test]
    fn test_is_within_unbounded() {
        let range = CallRange { start: Bound::Unbounded, end: Bound::Unbounded };
        assert!(range.is_within(0));
        assert!(range.is_within(100));
    }

    #[test]
    fn test_is_within_upper_unbounded() {
        let range = CallRange { start: Bound::Included(2), end: Bound::Unbounded };
        assert!(!range.is_within(1));
        assert!(range.is_within(2));
        assert!(range.is_within(3));
        assert!(range.is_within(100));
    }

    #[test]
    fn test_is_within_lower_unbounded() {
        let range = CallRange { start: Bound::Unbounded, end: Bound::Included(4) };
        assert!(range.is_within(0));
        assert!(range.is_within(2));
        assert!(range.is_within(4));
        assert!(!range.is_within(5));
    }

    #[test]
    fn test_is_at_max() {
        let range = CallRange { start: Bound::Included(2), end: Bound::Included(4) };
        assert!(!range.is_at_max(3));
        assert!(range.is_at_max(4));
        assert!(!range.is_at_max(5));

        let range = CallRange { start: Bound::Included(2), end: Bound::Excluded(4) };
        assert!(!range.is_at_max(2));
        assert!(range.is_at_max(3));
        assert!(!range.is_at_max(4));

        let range = CallRange { start: Bound::Included(2), end: Bound::Unbounded };
        assert!(!range.is_at_max(3));
        assert!(!range.is_at_max(100));
    }
}
